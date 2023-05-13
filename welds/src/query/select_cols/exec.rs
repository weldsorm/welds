use crate::connection::Connection;
use crate::query::clause::{DbParam, NextParam};
use crate::query::helpers::{build_tail, build_where_clauses, join_sql_parts};
use crate::query::select_cols::SelectBuilder;
use crate::table::{HasSchema, TableColumns, TableInfo};
use crate::writers::column::{ColumnWriter, DbColumnWriter};
use crate::writers::limit_skip::DbLimitSkipWriter;
use anyhow::Result;
use sqlx::database::HasArguments;
use sqlx::IntoArguments;

// ******************************************************************************************
// This file contains all the stuff added onto the SelectBuilder to allow it to run SELECTs
// ******************************************************************************************

impl<'schema, 'args, T, DB> SelectBuilder<'schema, T, DB>
where
    DB: sqlx::Database,
    T: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row> + HasSchema,
{
    fn sql_internal<'q>(
        &'q self,
        args: &mut Option<<DB as HasArguments<'schema>>::Arguments>,
    ) -> String
    where
        'schema: 'args,
        <DB as HasArguments<'schema>>::Arguments: IntoArguments<'args, DB>,
        DB: DbParam + DbColumnWriter + DbLimitSkipWriter,
        <T as HasSchema>::Schema: TableInfo + TableColumns<DB>,
    {
        let next_params = NextParam::new::<DB>();
        let wheres = self.qb.wheres.as_slice();
        let exists_in = self.qb.exist_ins.as_slice();
        let alias = &self.qb.alias;

        let mut wheres = build_where_clauses(&next_params, alias, args, wheres, exists_in);
        for j in &self.joins {
            j.append_where(&mut wheres, &next_params, args);
        }
        let where_sql = if wheres.is_empty() {
            None
        } else {
            Some(format!("WHERE ( {} )", wheres.join(" AND ")))
        };

        join_sql_parts(&[
            build_head_select(self),
            build_joins(self),
            where_sql,
            build_tail(&self.qb),
        ])
        .trim()
        .to_owned()
    }

    /// Get a copy of the SQL that will be executed when this query runs
    pub fn to_sql(&self) -> String
    where
        'schema: 'args,
        <DB as HasArguments<'schema>>::Arguments: IntoArguments<'args, DB>,
        DB: DbParam + DbColumnWriter + DbLimitSkipWriter,
        <T as HasSchema>::Schema: TableInfo + TableColumns<DB>,
    {
        self.sql_internal(&mut None)
    }

    /// Executes the query in the database returning the results
    pub async fn run<'q, 'c, C>(&'q self, exec: &'c C) -> Result<Vec<<DB as sqlx::Database>::Row>>
    where
        'schema: 'args,
        C: 'schema,
        C: Connection<DB>,
        <DB as HasArguments<'schema>>::Arguments: IntoArguments<'args, DB>,
        DB: DbParam + DbColumnWriter + DbLimitSkipWriter,
        <T as HasSchema>::Schema: TableInfo + TableColumns<DB>,
    {
        let mut args: Option<<DB as HasArguments>::Arguments> = Some(Default::default());
        let sql = self.sql_internal(&mut args);

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

        let rows = exec_hack.fetch_rows(sql, args.unwrap()).await?;
        Ok(rows)
    }
}

fn build_head_select<T, DB>(sb: &SelectBuilder<T, DB>) -> Option<String>
where
    T: HasSchema,
    DB: sqlx::Database + DbColumnWriter,
    <T as HasSchema>::Schema: TableInfo + TableColumns<DB>,
{
    let writer = ColumnWriter::new::<DB>();
    let mut head: Vec<&str> = Vec::default();
    head.push("SELECT");

    let mut cols: Vec<_> = Vec::default();
    let alias = &sb.qb.alias;

    // Add these columns
    for col in &sb.selects {
        let colname = writer.excape(&col.col_name);
        let fieldname = writer.excape(&col.field_name);
        if colname == fieldname {
            let col = format!("{}.{}", alias, colname);
            cols.push(col);
        } else {
            let col = format!("{}.{} as {}", alias, colname, fieldname);
            cols.push(col);
        }
    }

    // Add columns from joins
    for join in &sb.joins {
        join.append_columns(&mut cols);
    }

    let cols_text = cols.join(", ");
    head.push(&cols_text);

    head.push("FROM");
    let tn = <T as HasSchema>::Schema::identifier().join(".");
    let identifier = format!("{} {}", tn, alias);
    head.push(&identifier);
    Some(head.join(" "))
}

fn build_joins<T, DB>(sb: &SelectBuilder<T, DB>) -> Option<String>
where
    T: HasSchema,
    DB: sqlx::Database + DbColumnWriter,
    <T as HasSchema>::Schema: TableInfo + TableColumns<DB>,
{
    let mut list = Vec::default();
    let alias = &sb.qb.alias;
    // Add columns from joins
    for join in &sb.joins {
        join.append_jointable(&mut list, alias);
    }
    Some(list.join(" "))
}
