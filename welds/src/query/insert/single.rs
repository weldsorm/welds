use crate::errors::Result;
use crate::query::clause::{DbParam, NextParam};
use crate::table::{HasSchema, TableColumns, TableInfo, WriteToArgs};
use crate::writers::column::{ColumnWriter, DbColumnWriter};
use crate::writers::insert::{ColArg, DbInsertWriter, InsertWriter};
use sqlx::database::HasArguments;
use sqlx::IntoArguments;
use std::mem::swap;

pub async fn insert_one<'schema, 'args, 'e, DB, T, E>(
    buff: &'args mut String,
    obj: &mut T,
    exec: &'e mut E,
) -> Result<()>
where
    E: 'e,
    'schema: 'args,
    DB: sqlx::Database + DbParam + DbInsertWriter + DbColumnWriter,
    T: WriteToArgs<DB> + HasSchema + for<'r> sqlx::FromRow<'r, DB::Row>,
    <T as HasSchema>::Schema: TableInfo + TableColumns<DB>,
    &'e mut E: sqlx::Executor<'e, Database = DB>,
    <DB as HasArguments<'schema>>::Arguments: IntoArguments<'args, DB>,
{
    let mut args: <DB as HasArguments>::Arguments = Default::default();
    let args2: <DB as HasArguments>::Arguments = Default::default();
    let col_writer = ColumnWriter::new::<DB>();
    let next_params = NextParam::new::<DB>();
    let writer = InsertWriter::new::<DB>();

    let identifier = <<T as HasSchema>::Schema>::identifier();
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

    let (insert, select) = writer.write(identifier, &colargs, &columns, &pks);
    let has_select = select.is_some();
    *buff = format!("{}{}", insert, select.unwrap_or_default());

    let sql1 = &buff[..insert.len()];
    let sql2 = &buff[insert.len()..];

    if !has_select {
        let q = sqlx::query_with(sql1, args);
        if let Some(row) = q.fetch_optional(exec).await? {
            let mut t = T::from_row(&row)?;
            swap(&mut t, obj);
        };
    } else {
        let q1 = sqlx::query_with(sql1, args);
        let q2 = sqlx::query_with(sql2, args2);

        // HACK: fix if you can.
        //
        // We need to mut twice,
        // but the sqlx lifetimes hold the exec way longer than what is needed.
        // we are executing sequentially here, and not sharing these borrows.
        let exec_ptr: *const &mut E = &exec;
        let exec_hack1: &mut E = unsafe { *(exec_ptr as *mut &mut E) };
        let exec_hack2: &mut E = unsafe { *(exec_ptr as *mut &mut E) };

        q1.execute(exec_hack1).await.unwrap();
        if let Some(row) = q2.fetch_optional(exec_hack2).await? {
            let mut t = T::from_row(&row)?;
            swap(&mut t, obj);
        };
    }
    Ok(())
}
