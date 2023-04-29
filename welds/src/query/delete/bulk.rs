use super::super::{
    builder::QueryBuilder,
    clause::{DbParam, NextParam},
    helpers::{build_tail, build_where, join_sql_parts},
};
use crate::alias::TableAlias;
use crate::connection::Connection;
use crate::table::{HasSchema, TableColumns, TableInfo};
use crate::writers::column::DbColumnWriter;
use crate::writers::count::DbCountWriter;
use crate::writers::limit_skip::DbLimitSkipWriter;
use anyhow::Result;
use sqlx::database::HasArguments;
use sqlx::IntoArguments;

// ******************************************************************************************
// This file contains code on a Query builder to allow it to bulk delete
// ******************************************************************************************

impl<'schema, 'args, T, DB> QueryBuilder<'schema, T, DB>
where
    DB: sqlx::Database,
    T: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row> + HasSchema,
{
    /// Executes a `DELETE FROM ... `
    ///
    /// deletes all the resulting rows from the database
    pub async fn delete<'q, 'c, C>(&'q self, exec: &'c C) -> Result<()>
    where
        'schema: 'args,
        C: 'schema,
        C: Connection<DB>,
        <DB as HasArguments<'schema>>::Arguments: IntoArguments<'args, DB>,
        DB: DbParam + DbColumnWriter + DbLimitSkipWriter + DbCountWriter,
        <T as HasSchema>::Schema: TableInfo + TableColumns<DB>,
        i64: sqlx::Type<DB> + for<'r> sqlx::Decode<'r, DB>,
        usize: sqlx::ColumnIndex<<DB as sqlx::Database>::Row>,
    {
        let mut args: Option<<DB as HasArguments>::Arguments> = Some(Default::default());
        let next_params = NextParam::new::<DB>();
        let wheres = self.wheres.as_slice();
        let exists_in = self.exist_ins.as_slice();
        let alias = TableAlias::new();
        // Note: for deletes we can't alias the FROM tablename
        let fullname = <T as HasSchema>::Schema::identifier().join(".");
        alias.force_next(fullname);

        let sql = join_sql_parts(&[
            build_head_delete::<DB, <T as HasSchema>::Schema>(),
            build_where(&next_params, &alias, &mut args, wheres, exists_in),
            build_tail(self),
        ]);

        // lifetime hacks - Remove if you can
        // We know the use of sql and conn do not exceed the underlying call to fetch
        // sqlx if wants to hold the borrow for much longer than what is needed.
        // This hack prevents the borrow from exceeding the life of this call
        let sql_len = sql.len();
        let sqlp = sql.as_ptr();
        let sql_hack: &[u8] = unsafe { std::slice::from_raw_parts(sqlp, sql_len) };
        let sql: &str = std::str::from_utf8(sql_hack).unwrap();
        let exec_ptr: *const &C = &exec;
        let exec_hack: &mut C = unsafe { *(exec_ptr as *mut &mut C) };
        exec_hack.execute(sql, args.unwrap()).await?;

        Ok(())
    }
}

fn build_head_delete<DB, S>() -> Option<String>
where
    DB: sqlx::Database + DbColumnWriter + DbCountWriter,
    S: TableInfo + TableColumns<DB>,
{
    let identifier = S::identifier().join(".");
    //let identifier = format!("{} {}", tn, &tablealias);
    Some(format!("DELETE FROM {}", identifier))
}
