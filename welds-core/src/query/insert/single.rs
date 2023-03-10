use crate::errors::Result;
use crate::query::clause::{DbParam, NextParam};
use crate::table::{HasSchema, TableColumns, TableInfo, WriteToArgs};
use crate::writers::column::{ColumnWriter, DbColumnWriter};
use crate::writers::insert::{ColArg, DbInsertWriter, InsertWriter};
use sqlx::database::HasArguments;
use sqlx::IntoArguments;
use std::mem::swap;

pub async fn insert_one<'schema, 'args, 'e, 'eb, DB, T, E>(
    buff: &'args mut String,
    obj: &mut T,
    exec: E,
) -> Result<()>
where
    'schema: 'args,
    DB: sqlx::Database + DbParam + DbInsertWriter + DbColumnWriter,
    T: WriteToArgs<DB> + HasSchema + for<'r> sqlx::FromRow<'r, DB::Row>,
    <T as HasSchema>::Schema: TableInfo + TableColumns<DB>,
    E: sqlx::Executor<'e, Database = DB>,
    <DB as HasArguments<'schema>>::Arguments: IntoArguments<'args, DB>,
{
    let mut args: <DB as HasArguments>::Arguments = Default::default();
    let col_writer = ColumnWriter::new::<DB>();
    let next_params = NextParam::new::<DB>();
    let writer = InsertWriter::new::<DB>();

    let identifier = <<T as HasSchema>::Schema>::identifier();
    let columns = <<T as HasSchema>::Schema as TableColumns<DB>>::columns();
    let pks = <<T as HasSchema>::Schema as TableColumns<DB>>::primary_keys();

    let mut colargs = Vec::default();

    for col in &columns {
        if !pks.contains(&col) {
            obj.bind(col.name(), &mut args)?;
            let col = col_writer.excape(col.name());
            colargs.push(ColArg(col, next_params.next()));
        }
    }

    let sqls = writer.write(&identifier, &colargs, &columns);

    *buff = sqls;
    let q = sqlx::query_with(buff, args);
    if let Some(row) = q.fetch_optional(exec).await? {
        let mut t = T::from_row(&row)?;
        swap(&mut t, obj);
    };

    Ok(())
}
