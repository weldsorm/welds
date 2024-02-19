use crate::errors::Result;
use crate::errors::WeldsError::InsertFailed;
use crate::model_traits::UpdateFromRow;
use crate::model_traits::{HasSchema, TableColumns, TableInfo, WriteToArgs};
use crate::query::clause::ParamArgs;
use crate::writers::column::ColumnWriter;
use crate::writers::insert::{ColArg, InsertWriter};
use crate::writers::NextParam;
use welds_connections::Client;

pub async fn insert_one<T, C>(obj: &mut T, client: &C) -> Result<()>
where
    T: WriteToArgs + HasSchema,
    <T as HasSchema>::Schema: TableInfo + TableColumns,
    T: UpdateFromRow,
    C: Client,
{
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

    for col in &columns {
        if !pks.contains(col) {
            obj.bind(col.name(), &mut args)?;
            let col = col_writer.excape(col.name());
            colargs.push(ColArg(col, next_params.next()));
        }
    }

    let (insert, select) = writer.write(&identifier, &colargs, &columns, &pks);
    let has_select = select.is_some();

    let sql = format!("{}{}", insert, select.unwrap_or_default());
    let sql1 = &sql[..insert.len()];
    let sql2 = &sql[insert.len()..];
    let mut stamts = vec![(sql1, args)];

    if has_select {
        stamts.push((sql2, args2));
    }

    let mut rows = Vec::default();

    for (statement, args) in stamts {
        let mut out = client.fetch_rows(statement, &args).await?;
        rows.append(&mut out);
    }

    let row = rows.pop();
    let mut row = row.ok_or_else(|| InsertFailed(format!("{:?}", sql)))?;
    UpdateFromRow::update_from_row(obj, &mut row)?;

    Ok(())
}

#[cfg(test)]
mod tests;
