use crate::connection::Connection;
use crate::connection::Database;
use crate::query::clause::{DbParam, NextParam};
use crate::table::{HasSchema, TableColumns, TableInfo, WriteToArgs};
use crate::writers::column::{ColumnWriter, DbColumnWriter};
use anyhow::{anyhow, Result};
use sqlx::database::HasArguments;
use sqlx::IntoArguments;

pub async fn update_one<'r, 'args, DB, T, C>(
    buff: &'r mut String,
    obj: &T,
    conn: &'r C,
) -> Result<()>
where
    DB: Database,
    T: WriteToArgs<DB> + HasSchema,
    <T as HasSchema>::Schema: TableInfo + TableColumns<DB>,
    C: Connection<DB>,
    <DB as HasArguments<'r>>::Arguments: IntoArguments<'args, DB>,
{
    let mut args: <DB as HasArguments>::Arguments = Default::default();
    let col_writer = ColumnWriter::new::<DB>();
    let next_params = NextParam::new::<DB>();

    let identifier = <<T as HasSchema>::Schema>::identifier().join(".");
    let columns = <<T as HasSchema>::Schema as TableColumns<DB>>::columns();
    let pks = <<T as HasSchema>::Schema as TableColumns<DB>>::primary_keys();
    if pks.is_empty() {
        return Err(anyhow!(crate::errors::WeldsError::NoPrimaryKey));
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

    conn.execute(buff, args).await?;

    Ok(())
}
