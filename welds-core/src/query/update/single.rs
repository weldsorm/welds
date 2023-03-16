use crate::errors::Result;
use crate::query::clause::{DbParam, NextParam};
use crate::table::{HasSchema, TableColumns, TableInfo, WriteToArgs};
use crate::writers::column::{ColumnWriter, DbColumnWriter};
use sqlx::database::HasArguments;
use sqlx::IntoArguments;

pub async fn update_one<'schema, 'args, 'e, DB, T, E>(
    buff: &'args mut String,
    obj: &T,
    exec: &'e mut E,
) -> Result<()>
where
    E: 'e,
    'schema: 'args,
    DB: sqlx::Database + DbParam + DbColumnWriter,
    T: WriteToArgs<DB> + HasSchema,
    <T as HasSchema>::Schema: TableInfo + TableColumns<DB>,
    &'e mut E: sqlx::Executor<'e, Database = DB>,
    <DB as HasArguments<'schema>>::Arguments: IntoArguments<'args, DB>,
{
    let mut args: <DB as HasArguments>::Arguments = Default::default();
    let col_writer = ColumnWriter::new::<DB>();
    let next_params = NextParam::new::<DB>();

    let identifier = <<T as HasSchema>::Schema>::identifier();
    let columns = <<T as HasSchema>::Schema as TableColumns<DB>>::columns();
    let pks = <<T as HasSchema>::Schema as TableColumns<DB>>::primary_keys();
    if pks.is_empty() {
        return Err(crate::errors::WeldsError::NoPrimaryKey);
    }
    let mut sets = Vec::default();

    for col in columns {
        if !pks.contains(&col) {
            obj.bind(col.name(), &mut args)?;
            let p = next_params.next();
            let colname = col_writer.excape(col.name());
            sets.push(format!("{}={}", colname, p));
        }
    }
    if sets.is_empty() {
        return Ok(());
    }
    let mut wheres = Vec::default();
    for col in pks {
        obj.bind(col.name(), &mut args)?;
        let p = next_params.next();
        let colname = col_writer.excape(col.name());
        wheres.push(format!("{}={}", colname, p));
    }

    let sets = sets.join(", ");
    let wheres = wheres.join(" AND ");

    *buff = format!("UPDATE {} SET {} where {}", identifier, sets, wheres);

    eprintln!("SQL: {:?}", buff);
    let q = sqlx::query_with(buff, args);
    q.execute(exec).await?;

    Ok(())
}
