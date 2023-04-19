use crate::connection::Connection;
use crate::errors::WeldsError::InsertFailed;
use crate::query::clause::{DbParam, NextParam};
use crate::table::{HasSchema, TableColumns, TableInfo, WriteToArgs};
use crate::writers::column::{ColumnWriter, DbColumnWriter};
use crate::writers::insert::{ColArg, DbInsertWriter, InsertWriter};
use anyhow::Result;
use sqlx::database::HasArguments;
use sqlx::IntoArguments;
use std::mem::swap;

pub async fn insert_one<'r, 'args, DB, T, C>(
    buff: &'r mut String,
    obj: &mut T,
    conn: &'r C,
) -> Result<()>
where
    DB: sqlx::Database + DbParam + DbInsertWriter + DbColumnWriter,
    T: WriteToArgs<DB> + HasSchema + for<'fr> sqlx::FromRow<'fr, DB::Row>,
    <T as HasSchema>::Schema: TableInfo + TableColumns<DB>,
    C: Connection<DB>,
    <DB as HasArguments<'r>>::Arguments: IntoArguments<'args, DB>,
{
    let mut args: <DB as HasArguments>::Arguments = Default::default();
    let args2: <DB as HasArguments>::Arguments = Default::default();
    let col_writer = ColumnWriter::new::<DB>();
    let next_params = NextParam::new::<DB>();
    let writer = InsertWriter::new::<DB>();

    let identifier = <<T as HasSchema>::Schema>::identifier().join(".");
    let columns = <<T as HasSchema>::Schema as TableColumns<DB>>::columns();
    let pks = <<T as HasSchema>::Schema as TableColumns<DB>>::primary_keys();

    let mut colargs = Vec::default();

    for col in &columns {
        if !pks.contains(col) {
            obj.bind(col.name(), &mut args)?;
            let col = col_writer.excape(col.name());
            colargs.push(ColArg(col, next_params.next()));
        }
    }

    let (insert, select) = writer.write(&identifier, &colargs, &columns, &pks);
    let has_select = select.is_some();

    *buff = format!("{}{}", insert, select.unwrap_or_default());
    let sql1 = &buff[..insert.len()];
    let sql2 = &buff[insert.len()..];
    let mut stamts = vec![(sql1, args)];

    if has_select {
        stamts.push((sql2, args2));
    }

    let row = conn.fetch_many_rows(stamts).await?.pop();
    let row = row.ok_or_else(|| InsertFailed(format!("{:?}", buff)))?;
    let mut t = T::from_row(&row)?;
    swap(&mut t, obj);

    Ok(())
}
