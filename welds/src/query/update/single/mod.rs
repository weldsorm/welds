use crate::errors::{Result, WeldsError};
use crate::model_traits::hooks::{AfterUpdate, BeforeUpdate};
use crate::model_traits::{HasSchema, TableColumns, TableInfo, UpdateFromRow, WriteToArgs};
use crate::query::clause::ParamArgs;
use crate::writers::ColumnWriter;
use crate::writers::NextParam;
use welds_connections::Client;

pub async fn update_one<T>(obj: &mut T, client: &dyn Client) -> Result<()>
where
    T: WriteToArgs + HasSchema,
    <T as HasSchema>::Schema: TableInfo + TableColumns,
    T: UpdateFromRow,
    T: AfterUpdate + BeforeUpdate,
{
    BeforeUpdate::before(obj).await?;

    let syntax = client.syntax();
    let mut args: ParamArgs = Vec::default();
    let col_writer = ColumnWriter::new(syntax);
    let next_params = NextParam::new(syntax);

    let identifier = <<T as HasSchema>::Schema>::identifier().join(".");
    let columns = <<T as HasSchema>::Schema as TableColumns>::columns();
    let pks = <<T as HasSchema>::Schema as TableColumns>::primary_keys();
    if pks.is_empty() {
        return Err(WeldsError::NoPrimaryKey);
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

    let sql = format!("UPDATE {} SET {} where {}", identifier, sets, wheres);

    client.execute(&sql, &args).await?;

    AfterUpdate::after(obj).await.ok();
    Ok(())
}

#[cfg(test)]
mod tests;
