use super::clause::{DbParam, NextParam};
use crate::alias::TableAlias;
use crate::connection::Connection;
use crate::errors::Result;
use crate::query::clause::exists::ExistIn;
use crate::query::clause::{AsFieldName, ClauseAdder, OrderBy};
use crate::relations::{HasRelations, Relationship};
use crate::state::DbState;
use crate::table::{HasSchema, TableColumns, TableInfo, UniqueIdentifier};
use crate::writers::column::{ColumnWriter, DbColumnWriter};
use crate::writers::count::{CountWriter, DbCountWriter};
use crate::writers::limit_skip::DbLimitSkipWriter;
use sqlx::database::HasArguments;
use sqlx::IntoArguments;
use sqlx::Row;
use std::marker::PhantomData;

/// An un-executed Query.
///
/// Build out a query that can be executed on the database.
///
/// Can be chained with other queries to make more complex queries.
///
/// Can be mapped into other queries to  make more complex queries.
pub struct SelectBuilder<'schema, T, DB: sqlx::Database> {
    _t: PhantomData<T>,
    pub(crate) wheres: Vec<Box<dyn ClauseAdder<'schema, DB>>>,
    pub(crate) exist_ins: Vec<ExistIn<'schema, DB>>,
    pub(crate) limit: Option<i64>,
    pub(crate) offset: Option<i64>,
    pub(crate) orderby: Vec<OrderBy>,
}

impl<'schema, T, DB> Default for SelectBuilder<'schema, T, DB>
where
    DB: sqlx::Database,
    T: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row> + HasSchema,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'schema, 'args, T, DB> SelectBuilder<'schema, T, DB>
where
    DB: sqlx::Database,
    T: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row> + HasSchema,
{
    pub fn new() -> Self {
        Self {
            _t: Default::default(),
            wheres: Vec::default(),
            limit: None,
            offset: None,
            orderby: Vec::default(),
            exist_ins: Default::default(),
        }
    }

    /// Filter the results returned by this query.
    /// Used when you want to filter on the columns of this table.
    pub fn where_col(
        mut self,
        lam: impl Fn(<T as HasSchema>::Schema) -> Box<dyn ClauseAdder<'schema, DB>>,
    ) -> Self
    where
        <T as HasSchema>::Schema: Default,
    {
        let qba = lam(Default::default());
        self.wheres.push(qba);
        self
    }

    /// Add a query to this query (JOIN on a relationship)
    /// results on a query that is filtered using the results of both queries
    pub fn where_relation<R, Ship>(
        mut self,
        relationship: impl Fn(<T as HasRelations>::Relation) -> Ship,
        filter: SelectBuilder<'schema, R, DB>,
    ) -> Self
    where
        DB: sqlx::Database + DbLimitSkipWriter,
        T: HasRelations + UniqueIdentifier<DB>,
        Ship: Relationship<R>,
        R: HasSchema + UniqueIdentifier<DB>,
        R: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row> + HasSchema,
        <R as HasSchema>::Schema: TableInfo + TableColumns<DB>,
        <T as HasRelations>::Relation: Default,
    {
        let ship = relationship(Default::default());
        let out_col = ship.my_key::<DB, R, T>();
        let inner_tn = <R as HasSchema>::Schema::identifier().to_owned();
        let inner_col = ship.their_key::<DB, R, T>();
        let exist_in = ExistIn::<'schema, DB>::new(filter, out_col, inner_tn, inner_col);
        self.exist_ins.push(exist_in);
        self
    }

    /// Results in a query that is mapped into the query of one of its relationships
    pub fn map_query<R, Ship>(
        self,
        relationship: impl Fn(<T as HasRelations>::Relation) -> Ship,
    ) -> SelectBuilder<'schema, R, DB>
    where
        DB: sqlx::Database + DbLimitSkipWriter,
        T: HasRelations + UniqueIdentifier<DB>,
        Ship: Relationship<R>,
        R: HasSchema + UniqueIdentifier<DB>,
        R: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row> + HasSchema,
        <R as HasSchema>::Schema: TableInfo + TableColumns<DB>,
        <T as HasRelations>::Relation: Default,
    {
        let ship = relationship(Default::default());
        let mut sb: SelectBuilder<R, DB> = SelectBuilder::new();

        let out_col = ship.their_key::<DB, R, T>();
        let inner_tn = <T as HasSchema>::Schema::identifier().to_owned();
        let inner_col = ship.my_key::<DB, R, T>();
        let exist_in = ExistIn::<'schema, DB>::new(self, out_col, inner_tn, inner_col);

        sb.exist_ins.push(exist_in);
        sb
    }

    /// Limit the number of rows returned by this query
    pub fn limit(mut self, x: i64) -> Self {
        self.limit = Some(x);
        self
    }

    /// Offset the starting point for the results returned by this query
    pub fn offset(mut self, x: i64) -> Self {
        self.offset = Some(x);
        self
    }

    /// Order the results of the query by a given column
    ///
    /// multiple calls will result in multiple OrderBys
    pub fn order_by_desc<FN: AsFieldName>(
        mut self,
        lam: impl Fn(<T as HasSchema>::Schema) -> FN,
    ) -> Self {
        let field = lam(Default::default());
        let fieldname = field.fieldname();
        self.orderby.push(OrderBy::new(fieldname, "DESC"));
        self
    }

    /// Order the results of the query by a given column
    ///
    /// multiple calls will result in multiple OrderBys
    pub fn order_by_asc<FN: AsFieldName>(
        mut self,
        lam: impl Fn(<T as HasSchema>::Schema) -> FN,
    ) -> Self {
        let field = lam(Default::default());
        let fieldname = field.fieldname();
        self.orderby.push(OrderBy::new(fieldname, "ASC"));
        self
    }

    /// Get a copy of the SQL that will be executed when this query runs
    pub fn to_sql(&self) -> String
    where
        <DB as HasArguments<'schema>>::Arguments: IntoArguments<'args, DB>,
        DB: DbParam + DbColumnWriter + DbLimitSkipWriter + DbCountWriter,
        <T as HasSchema>::Schema: TableInfo + TableColumns<DB>,
    {
        let mut args: Option<<DB as HasArguments>::Arguments> = None;
        let next_params = NextParam::new::<DB>();
        let wheres = self.wheres.as_slice();
        let exists_in = self.exist_ins.as_slice();
        let alias = TableAlias::new();
        let self_tablealias = alias.peek();

        join_sql_parts(&[
            build_head_select::<DB, <T as HasSchema>::Schema>(self_tablealias),
            build_where(&next_params, &alias, &mut args, wheres, exists_in),
            build_tail(self),
        ])
    }

    /// Executes a `select count(...) FROM ... `
    ///
    /// Counts the results of your query in the database.
    pub async fn count<'q, 'c, C>(&'q self, exec: &'c C) -> Result<u64>
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
        let self_tablealias = alias.peek();

        let sql = join_sql_parts(&[
            build_head_count::<DB, <T as HasSchema>::Schema>(self_tablealias),
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

        let rows = exec_hack.fetch_rows(sql, args.unwrap()).await?;
        let row = rows.get(0).unwrap();
        let count: i64 = row.try_get(0)?;
        Ok(count as u64)
    }

    /// Executes the query in the database returning the results
    pub async fn run<'q, 'c, C>(&'q self, exec: &'c C) -> Result<Vec<DbState<T>>>
    where
        'schema: 'args,
        C: 'schema,
        C: Connection<DB>,
        <DB as HasArguments<'schema>>::Arguments: IntoArguments<'args, DB>,
        DB: DbParam + DbColumnWriter + DbLimitSkipWriter,
        <T as HasSchema>::Schema: TableInfo + TableColumns<DB>,
    {
        let mut args: Option<<DB as HasArguments>::Arguments> = Some(Default::default());
        let next_params = NextParam::new::<DB>();
        let wheres = self.wheres.as_slice();
        let exists_in = self.exist_ins.as_slice();
        let alias = TableAlias::new();
        let self_tablealias = alias.peek();

        let sql = join_sql_parts(&[
            build_head_select::<DB, <T as HasSchema>::Schema>(self_tablealias),
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

        let data = exec_hack
            .fetch_all(sql, args.unwrap())
            .await?
            .drain(..)
            .map(|d| DbState::db_loaded(d))
            .collect();

        Ok(data)
    }
}

fn join_sql_parts(parts: &[Option<String>]) -> String {
    // Join al the parts into
    let sql: Vec<&str> = parts
        .iter()
        .filter_map(|x| x.as_ref().map(|x| x.as_str()))
        .collect();
    let sql: String = sql.as_slice().join(" ");
    sql
}

fn build_where<'schema, 'args, DB>(
    next_params: &NextParam,
    alias: &TableAlias,
    args: &mut Option<<DB as HasArguments<'schema>>::Arguments>,
    wheres: &[Box<dyn ClauseAdder<'schema, DB>>],
    exist_ins: &[ExistIn<'schema, DB>],
) -> Option<String>
where
    DB: sqlx::Database + DbLimitSkipWriter,
    <DB as HasArguments<'schema>>::Arguments: IntoArguments<'args, DB>,
{
    let mut where_sql: Vec<String> = Vec::default();

    for clause in wheres {
        if let Some(args) = args {
            clause.bind(args);
        }
        if let Some(p) = clause.clause(alias, next_params) {
            where_sql.push(p);
        }
    }

    let self_tablealias = alias.peek();
    for clause in exist_ins {
        if let Some(args) = args {
            clause.bind(args);
        }
        alias.bump();
        clause.set_outer_tablealias(&self_tablealias);
        if let Some(p) = clause.clause(alias, next_params) {
            where_sql.push(p);
        }
    }

    if where_sql.is_empty() {
        return None;
    }
    Some(format!("WHERE ( {} )", where_sql.join(" AND ")))
}

fn build_tail<T, DB>(select: &SelectBuilder<T, DB>) -> Option<String>
where
    T: HasSchema,
    DB: sqlx::Database + DbLimitSkipWriter,
    <T as HasSchema>::Schema: TableInfo + TableColumns<DB>,
{
    super::tail::write::<DB>(&select.limit, &select.offset, &select.orderby)
}

fn build_head_select<DB, S>(tablealias: String) -> Option<String>
where
    DB: sqlx::Database + DbColumnWriter,
    S: TableInfo + TableColumns<DB>,
{
    let writer = ColumnWriter::new::<DB>();
    let mut head: Vec<&str> = Vec::default();
    head.push("SELECT");
    let cols_info = S::columns();
    let cols: Vec<_> = cols_info
        .iter()
        .map(|col| writer.write(&tablealias, col))
        .collect();
    let cols = cols.join(", ");
    head.push(&cols);
    head.push("FROM");
    let identifier = format!("{} {}", S::identifier(), tablealias);
    head.push(&identifier);
    Some(head.join(" "))
}

fn build_head_count<DB, S>(tablealias: String) -> Option<String>
where
    DB: sqlx::Database + DbColumnWriter + DbCountWriter,
    S: TableInfo + TableColumns<DB>,
{
    let identifier = format!("{} {}", S::identifier(), &tablealias);
    let cw = CountWriter::new::<DB>();
    let count_star = cw.count(Some(&tablealias), Some("*"));
    Some(format!("SELECT {} FROM {}", count_star, identifier))
}
