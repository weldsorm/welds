use crate::errors::Result;
use crate::errors::WeldsError;
use crate::model_traits::hooks::{AfterDelete, BeforeDelete};
use crate::model_traits::{HasSchema, TableColumns, TableInfo, WriteToArgs};
use crate::query::clause::ParamArgs;
use crate::writers::ColumnWriter;
use crate::writers::NextParam;
use welds_connections::Client;

pub mod bulk;

pub async fn delete_one<T>(obj: &T, client: &dyn Client) -> Result<()>
where
    T: HasSchema + WriteToArgs,
    <T as HasSchema>::Schema: TableInfo + TableColumns,
    T: AfterDelete + BeforeDelete,
{
    BeforeDelete::before(obj).await?;
    let syntax = client.syntax();
    let col_writer = ColumnWriter::new(syntax);
    let next_params = NextParam::new(syntax);
    let identifier = <<T as HasSchema>::Schema>::identifier().join(".");

    let pks = <<T as HasSchema>::Schema as TableColumns>::primary_keys();
    if pks.is_empty() {
        return Err(WeldsError::NoPrimaryKey);
    }

    let mut args: ParamArgs = Vec::default();
    let mut wheres = Vec::default();
    for col in pks {
        obj.bind(col.name(), &mut args)?;
        let p = next_params.next();
        let colname = col_writer.excape(col.name());
        wheres.push(format!("{}={}", colname, p));
    }

    let wheres = wheres.join(" AND ");

    let sql = format!("DELETE FROM {} where {}", identifier, wheres);

    client.execute(&sql, &args).await?;
    AfterDelete::after(obj).await.ok();

    Ok(())
}

#[cfg(test)]
mod tests;
