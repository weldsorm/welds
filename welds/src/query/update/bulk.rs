use crate::connection::Connection;
use crate::errors::Result;
use crate::query::builder::QueryBuilder;
use crate::query::clause::AsFieldName;
use crate::query::clause::ClauseAdder;
use crate::query::clause::{wherein::WhereIn, DbParam, NextParam};
use crate::query::helpers::{build_where, join_sql_parts};
use crate::table::UniqueIdentifier;
use crate::table::{HasSchema, TableColumns, TableInfo};
use crate::writers::column::{ColumnWriter, DbColumnWriter};
use crate::writers::count::DbCountWriter;
use crate::writers::limit_skip::DbLimitSkipWriter;
use sqlx::database::HasArguments;
use sqlx::IntoArguments;
use std::marker::PhantomData;

/// An un-executed Sql Update.
///
/// Build out a sql statement that will update the database in bulk

pub struct UpdateBuilder<'schema, T, DB: sqlx::Database> {
    _t: PhantomData<T>,
    pub(crate) query_builder: QueryBuilder<'schema, T, DB>,
    pub(crate) sets: Vec<Box<dyn ClauseAdder<'schema, DB>>>,
}

impl<'schema, 'args, T, DB> UpdateBuilder<'schema, T, DB>
where
    DB: sqlx::Database,
    T: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row> + HasSchema,
{
    pub(crate) fn new(query_builder: QueryBuilder<'schema, T, DB>) -> Self {
        Self {
            _t: Default::default(),
            sets: Vec::default(),
            query_builder,
        }
    }

    /// Filter the results returned by this query.
    /// Used when you want to filter on the columns of this table.
    pub fn set<V, FIELD>(
        mut self,
        lam: impl Fn(<T as HasSchema>::Schema) -> FIELD,
        value: impl Into<V>,
    ) -> Self
    where
        DB: DbColumnWriter,
        <T as HasSchema>::Schema: Default,
        FIELD: AsFieldName<V>,
        V: for<'r> sqlx::Encode<'r, DB> + sqlx::Type<DB> + Send + Clone,
        V: 'static,
    {
        let val: V = value.into();
        let field = lam(Default::default());
        let col_raw = field.colname();
        let col_writer = ColumnWriter::new::<DB>();
        let colname = col_writer.excape(col_raw);
        self.sets.push(Box::new(SetColVal { col: colname, val }));
        self
    }

    /// Get a copy of the SQL that will be executed when this query runs
    pub fn to_sql(&self) -> String
    where
        'schema: 'args,
        <DB as HasArguments<'schema>>::Arguments: IntoArguments<'args, DB>,
        DB: DbParam + DbColumnWriter + DbLimitSkipWriter + DbCountWriter,
        <T as HasSchema>::Schema: UniqueIdentifier<DB> + TableInfo + TableColumns<DB>,
    {
        self.sql_internal(&mut None)
    }

    fn sql_internal<'q>(
        &'q self,
        args: &mut Option<<DB as HasArguments<'schema>>::Arguments>,
    ) -> String
    where
        'schema: 'args,
        <DB as HasArguments<'schema>>::Arguments: IntoArguments<'args, DB>,
        DB: DbParam + DbColumnWriter + DbLimitSkipWriter,
        <T as HasSchema>::Schema: UniqueIdentifier<DB> + TableInfo + TableColumns<DB>,
    {
        let next_params = NextParam::new::<DB>();
        let sets = self.sets.as_slice();
        let alias = <T as HasSchema>::Schema::identifier().join(".");

        join_sql_parts(&[
            build_head::<DB, <T as HasSchema>::Schema>(&next_params, &alias, args, sets),
            build_where_update(&next_params, &alias, args, &self.query_builder),
        ])
    }

    /// Executes the query in the database Bulk updating the values
    pub async fn run<'q, 'c, C>(&'q self, exec: &'c C) -> Result<()>
    where
        'schema: 'args,
        C: 'schema,
        C: Connection<DB>,
        <DB as HasArguments<'schema>>::Arguments: IntoArguments<'args, DB>,
        DB: DbParam + DbColumnWriter + DbLimitSkipWriter,
        <T as HasSchema>::Schema: UniqueIdentifier<DB> + TableInfo + TableColumns<DB>,
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

        exec_hack.execute(sql, args.unwrap()).await?;
        Ok(())
    }
}

fn build_head<'schema, DB, S>(
    next_params: &NextParam,
    alias: &str,
    args: &mut Option<<DB as HasArguments<'schema>>::Arguments>,
    sets: &[Box<dyn ClauseAdder<'schema, DB>>],
) -> Option<String>
where
    DB: sqlx::Database + DbColumnWriter,
    S: TableInfo + TableColumns<DB>,
{
    let tn = S::identifier().join(".");

    let mut set_parts: Vec<String> = Vec::default();

    for clause in sets {
        if let Some(args) = args {
            clause.bind(args);
        }
        if let Some(p) = clause.clause(alias, next_params) {
            set_parts.push(p);
        }
    }
    let set_sql = set_parts.join(", ");

    Some(format!("UPDATE {tn} SET {sets}", tn = tn, sets = set_sql))
}

pub struct SetColVal<T> {
    pub col: String,
    pub val: T,
}

impl<'args, T, DB> ClauseAdder<'args, DB> for SetColVal<T>
where
    DB: sqlx::Database,
    T: 'args + Clone + Send + sqlx::Type<DB> + sqlx::Encode<'args, DB>,
{
    fn bind(&self, args: &mut <DB as HasArguments<'args>>::Arguments) {
        use sqlx::Arguments;
        args.add(self.val.clone());
    }

    fn clause(&self, _alias: &str, next_params: &NextParam) -> Option<String> {
        let sql = format!("{}={}", self.col, next_params.next());
        Some(sql)
    }
}

pub(crate) fn build_where_update<'schema, 'args, DB, T>(
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
    // If we have a limit, we need to wrap the wheres in an IN clause
    // this is to limit the number of row to that will be updated
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

    // use fulltable name for alias when updating
    let tableparts = T::Schema::identifier();
    let outer_tablealias = tableparts.join(".");

    if let Some(p) = w_in.clause(&outer_tablealias, next_params) {
        where_sql.push(p);
    }

    if where_sql.is_empty() {
        return None;
    }
    Some(format!("WHERE ( {} )", where_sql.join(" AND ")))
}
