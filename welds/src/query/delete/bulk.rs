use super::super::{
    builder::QueryBuilder,
    clause::{wherein::WhereIn, ClauseAdder, DbParam, NextParam},
    helpers::{build_where, join_sql_parts},
};
use crate::connection::Connection;
use crate::table::UniqueIdentifier;
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
    /// The SQL to delete a `DELETE FROM ... `
    ///
    /// return SQL to delete all the resulting rows from the database
    pub fn delete_sql<'q>(&'q self) -> String
    where
        'schema: 'args,
        <DB as HasArguments<'schema>>::Arguments: IntoArguments<'args, DB>,
        DB: DbParam + DbColumnWriter + DbLimitSkipWriter + DbCountWriter,
        <T as HasSchema>::Schema: TableInfo + TableColumns<DB> + UniqueIdentifier<DB>,
        i64: sqlx::Type<DB> + for<'r> sqlx::Decode<'r, DB>,
        usize: sqlx::ColumnIndex<<DB as sqlx::Database>::Row>,
    {
        self.delete_sql_internal(&mut None)
    }

    fn delete_sql_internal<'q>(
        &'q self,
        args: &mut Option<<DB as HasArguments<'schema>>::Arguments>,
    ) -> String
    where
        'schema: 'args,
        <DB as HasArguments<'schema>>::Arguments: IntoArguments<'args, DB>,
        DB: DbParam + DbColumnWriter + DbLimitSkipWriter + DbCountWriter,
        <T as HasSchema>::Schema: TableInfo + TableColumns<DB> + UniqueIdentifier<DB>,
        i64: sqlx::Type<DB> + for<'r> sqlx::Decode<'r, DB>,
        usize: sqlx::ColumnIndex<<DB as sqlx::Database>::Row>,
    {
        let next_params = NextParam::new::<DB>();

        // Note: for deletes we can't alias the FROM tablename
        let alias = <T as HasSchema>::Schema::identifier().join(".");

        join_sql_parts(&[
            build_head_delete::<DB, <T as HasSchema>::Schema>(),
            build_where_delete(&next_params, &alias, args, self),
        ])
    }

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
        <T as HasSchema>::Schema: TableInfo + TableColumns<DB> + UniqueIdentifier<DB>,
        i64: sqlx::Type<DB> + for<'r> sqlx::Decode<'r, DB>,
        usize: sqlx::ColumnIndex<<DB as sqlx::Database>::Row>,
    {
        let mut args: Option<<DB as HasArguments>::Arguments> = Some(Default::default());
        let sql = self.delete_sql_internal(&mut args);

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
    Some(format!("DELETE FROM {}", identifier))
}

pub(crate) fn build_where_delete<'schema, 'args, DB, T>(
    next_params: &NextParam,
    alias: &str,
    args: &mut Option<<DB as HasArguments<'schema>>::Arguments>,
    qb: &QueryBuilder<'schema, T, DB>,
) -> Option<String>
where
    'schema: 'args,
    T: HasSchema,
    <T as HasSchema>::Schema: UniqueIdentifier<DB> + TableInfo + TableColumns<DB>,
    DB: sqlx::Database + DbLimitSkipWriter + DbColumnWriter,
    <DB as HasArguments<'schema>>::Arguments: IntoArguments<'args, DB>,
{
    // If we have a limit, we need to wrap the wheres in an IN clause to
    // we can limit the number of row to delete
    if qb.limit.is_none() {
        let wheres = qb.wheres.as_slice();
        let exists_in = qb.exist_ins.as_slice();
        return build_where(next_params, alias, args, wheres, exists_in);
    }

    let mut where_sql: Vec<String> = Vec::default();
    let w_in = WhereIn::new(qb);

    if let Some(args) = args {
        w_in.bind(args);
    }
    if let Some(p) = w_in.clause(alias, next_params) {
        where_sql.push(p);
    }
    if where_sql.is_empty() {
        return None;
    }
    Some(format!("WHERE ( {} )", where_sql.join(" AND ")))
}
