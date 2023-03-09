use super::clause::{DbParam, NextParam};
use crate::errors::Result;
use crate::table::{HasSchema, TableColumns, TableInfo, WriteToArgs};
use sqlx::database::HasArguments;
use sqlx::IntoArguments;

pub async fn update_one<'schema, 'args, 'e, DB, T, E>(
    buff: &'args mut String,
    obj: &T,
    exec: E,
) -> Result<()>
where
    'schema: 'args,
    DB: sqlx::Database + DbParam, //'q: 'args,
    T: WriteToArgs<DB> + HasSchema,
    <T as HasSchema>::Schema: TableInfo + TableColumns<DB>,
    E: sqlx::Executor<'e, Database = DB>,
    <DB as HasArguments<'schema>>::Arguments: IntoArguments<'args, DB>,
{
    let mut args: <DB as HasArguments>::Arguments = Default::default();
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
            sets.push(format!("\"{}\"={}", col.name(), p));
        }
    }
    if sets.is_empty() {
        return Ok(());
    }
    let mut wheres = Vec::default();
    for col in pks {
        obj.bind(col.name(), &mut args)?;
        let p = next_params.next();
        wheres.push(format!("\"{}\"={}", col.name(), p));
    }

    let sets = sets.join(", ");
    let wheres = wheres.join(" AND ");

    *buff = format!("UPDATE {} SET {} where {}", identifier, sets, wheres);

    let q = sqlx::query_with(buff, args);
    q.execute(exec).await?;

    Ok(())
}
