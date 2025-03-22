use crate::errors::Result;
use crate::errors::WeldsError::InsertFailed;
use crate::model_traits::hooks::{AfterCreate, BeforeCreate};
use crate::model_traits::{ColumnDefaultCheck, UpdateFromRow};
use crate::model_traits::{HasSchema, TableColumns, TableInfo, WriteToArgs};
use crate::query::clause::ParamArgs;
use crate::writers::column::ColumnWriter;
use crate::writers::insert::{ColArg, InsertWriter};
use crate::writers::NextParam;
use crate::Row;
use welds_connections::Client;
use welds_connections::Fetch;

pub async fn insert_one<T>(obj: &mut T, client: &dyn Client) -> Result<()>
where
    T: WriteToArgs + HasSchema + ColumnDefaultCheck,
    <T as HasSchema>::Schema: TableInfo + TableColumns,
    T: UpdateFromRow,
    T: BeforeCreate + AfterCreate,
{
    BeforeCreate::before(obj).await?;

    let syntax = client.syntax();
    let mut args: ParamArgs = Vec::default();
    let args2: ParamArgs = Vec::default();

    let col_writer = ColumnWriter::new(syntax);
    let next_params = NextParam::new(syntax);
    let writer = InsertWriter::new(syntax);

    let identifier = <<T as HasSchema>::Schema>::identifier().join(".");
    let columns = <<T as HasSchema>::Schema as TableColumns>::columns();
    let pks = <<T as HasSchema>::Schema as TableColumns>::primary_keys();

    let mut colargs = Vec::default();
    let mut id_return_required = true;

    for col in &columns {
        let pk = pks.iter().find(|p| p == &col);

        match pk {
            None => {
                // column isn't PK just insert it
                obj.bind(col.name(), &mut args)?;
                let col = col_writer.excape(col.name());
                colargs.push(ColArg(col, next_params.next()));
            }
            Some(_) => {
                // If the column is the default value,
                // the user is expected an ID to be returned
                //
                // If it is NOT the default value we need to include
                // it in the query for insertion
                if !obj.col_is_default(col.name())? {
                    id_return_required = false;
                    obj.bind(col.name(), &mut args)?;
                    let col = col_writer.excape(col.name());
                    colargs.push(ColArg(col, next_params.next()));
                }
            }
        }
    }

    let (insert, select) = writer.write(&identifier, &colargs, &columns, &pks);

    let mut statements = vec![Fetch {
        sql: &insert,
        params: &args,
    }];

    // If this insert needs a second select command to get the id, add it to the vec of sql to run
    let sql2: String;
    if id_return_required {
        if let Some(select) = select {
            sql2 = select.to_owned();
            statements.push(Fetch {
                sql: &sql2,
                params: &args2,
            })
        }
    }

    // WARNING: these statements MUST be ran on the same DB connection in the pool
    // If this isn't done, you will not get back the last_id.
    // That is why we are using fetch_many
    let mut datasets = client.fetch_many(&statements).await?;
    let mut rows: Vec<Row> = datasets.drain(..).flatten().collect();

    // If we are providing the DB with the ID, (string/uuid) it doesn't need to return the id, and will not
    if !id_return_required {
        AfterCreate::after(obj).await.ok();
        return Ok(());
    }

    let row = rows.pop();
    let mut row =
        row.ok_or_else(|| InsertFailed("Insert didn't return inserted ID/Row".to_owned()))?;
    UpdateFromRow::update_from_row(obj, &mut row)?;
    AfterCreate::after(obj).await.ok();

    Ok(())
}

#[cfg(test)]
mod tests;
