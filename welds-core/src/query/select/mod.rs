use super::clause::{DbParam, NextParam};
use crate::alias::TableAlias;
use crate::errors::Result;
use crate::query::clause::exists::ExistIn;
use crate::query::clause::orderby;
use crate::query::clause::{AsFieldName, ClauseAdder, OrderBy};
use crate::relations::{HasRelations, Relationship};
use crate::state::DbState;
use crate::table::{HasSchema, TableColumns, TableInfo, UniqueIdentifier};
use crate::writers::column::{ColumnWriter, DbColumnWriter};
use crate::writers::count::{CountWriter, DbCountWriter};
use crate::writers::limit_skip::{DbLimitSkipWriter, LimitSkipWriter};
use sqlx::database::HasArguments;
use sqlx::query::{Query, QueryAs};
use sqlx::IntoArguments;
use sqlx::Row;
use std::collections::VecDeque;
use std::marker::PhantomData;

pub struct SelectBuilder<'schema, T, DB: sqlx::Database> {
    _t: PhantomData<T>,
    pub(crate) wheres: Vec<Box<dyn ClauseAdder<'schema, DB>>>,
    pub(crate) exist_ins: Vec<ExistIn<'schema, DB>>,
    limit: Option<i64>,
    offset: Option<i64>,
    orderby: Vec<OrderBy>,
    // these are needed for lifetime issues, remove if you can
    history: Vec<String>,
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
            history: Default::default(),
            exist_ins: Default::default(),
        }
    }

    pub fn where_col(
        mut self,
        lam: impl Fn(<T as HasSchema>::Schema) -> Box<dyn ClauseAdder<'schema, DB>>,
    ) -> Self
    where
        <T as HasSchema>::Schema: Default,
    {
        let mut qba = lam(Default::default());
        self.wheres.push(qba);
        self
    }

    //pub fn where_relation<R, Ship>(
    //    mut self,
    //    relationship: impl Fn(<T as HasRelations>::Relation) -> Ship,
    //    filter: SelectBuilder<'schema, R, DB>,
    //) -> Self
    //where
    //    T: HasRelations + UniqueIdentifier<DB>,
    //    Ship: Relationship<R>,
    //    R: HasSchema + UniqueIdentifier<DB>,
    //    R: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row> + HasSchema,
    //    <R as HasSchema>::Schema: TableInfo + TableColumns<DB>,
    //    <T as HasRelations>::Relation: Default,
    //{
    //    //let ship = relationship(Default::default());
    //    ////let mut sb: SelectBuilder<R, DB> = SelectBuilder::new();
    //    //sb.identifier_count = self.identifier_count + 1;
    //    //let out_tn = format!("t{}", sb.identifier_count);
    //    //let out_col = ship.their_key::<DB, R, T>();
    //    //let inner_ta = format!("t{}", self.identifier_count);
    //    //let inner_tn = <T as HasSchema>::Schema::identifier().to_owned();
    //    //let inner_col = ship.my_key::<DB, R, T>();
    //    //let exist_in =
    //    //    ExistIn::<'schema, DB>::new(self, out_tn, out_col, inner_tn, inner_ta, inner_col);
    //    self
    //}

    pub fn map_query<R, Ship>(
        self,
        relationship: impl Fn(<T as HasRelations>::Relation) -> Ship,
    ) -> SelectBuilder<'schema, R, DB>
    where
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

    pub fn limit(mut self, x: i64) -> Self {
        self.limit = Some(x);
        self
    }

    pub fn offset(mut self, x: i64) -> Self {
        self.offset = Some(x);
        self
    }

    pub fn order_by_desc<FN: AsFieldName>(
        mut self,
        lam: impl Fn(<T as HasSchema>::Schema) -> FN,
    ) -> Self {
        let field = lam(Default::default());
        let fieldname = field.fieldname();
        self.orderby.push(OrderBy::new(fieldname, "DESC"));
        self
    }

    pub fn order_by_asc<FN: AsFieldName>(
        mut self,
        lam: impl Fn(<T as HasSchema>::Schema) -> FN,
    ) -> Self {
        let field = lam(Default::default());
        let fieldname = field.fieldname();
        self.orderby.push(OrderBy::new(fieldname, "ASC"));
        self
    }

    pub fn to_sql(&mut self) -> String
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
            build_tail(&self),
        ])
    }

    pub async fn count<'q, 'e, E>(&'q mut self, exec: E) -> Result<u64>
    where
        'schema: 'args,
        E: sqlx::Executor<'e, Database = DB>,
        <DB as HasArguments<'schema>>::Arguments: IntoArguments<'args, DB>,
        i64: sqlx::Type<DB> + for<'r> sqlx::Decode<'r, DB>,
        usize: sqlx::ColumnIndex<<DB as sqlx::Database>::Row>,
        DB: DbParam + DbColumnWriter + DbLimitSkipWriter + DbCountWriter,
        <T as HasSchema>::Schema: TableInfo + TableColumns<DB>,
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
            build_tail(&self),
        ]);

        // lifetime hack
        // We know the SQL string is keep around until after execution is complete.
        self.history.push(sql);
        let sql = self.history.last().unwrap();
        let sql_len = sql.len();
        let sqlp = sql.as_ptr();
        let sql_hack: &[u8] = unsafe { std::slice::from_raw_parts(sqlp, sql_len) };
        let sql: &str = std::str::from_utf8(&sql_hack).unwrap();

        // Run the query
        let query: Query<DB, <DB as HasArguments>::Arguments> =
            sqlx::query_with(sql, args.unwrap());
        let row = query.fetch_one(exec).await?;
        let count: i64 = row.try_get(0)?;
        Ok(count as u64)
    }

    pub async fn run<'q, 'e, E>(&'q mut self, exec: E) -> Result<Vec<DbState<T>>>
    where
        'schema: 'args,
        E: sqlx::Executor<'e, Database = DB>,
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
            build_tail(&self),
        ]);

        // lifetime hack
        // We know the SQL string is keep around until after execution is complete.
        self.history.push(sql);
        let sql = self.history.last().unwrap();
        let sql_len = sql.len();
        let sqlp = sql.as_ptr();
        let sql_hack: &[u8] = unsafe { std::slice::from_raw_parts(sqlp, sql_len) };
        let sql: &str = std::str::from_utf8(&sql_hack).unwrap();

        // Run the query
        let q: QueryAs<DB, T, <DB as HasArguments>::Arguments> =
            sqlx::query_as_with(sql, args.unwrap());
        let data = q
            .fetch_all(exec)
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
    DB: sqlx::Database,
    <DB as HasArguments<'schema>>::Arguments: IntoArguments<'args, DB>,
{
    let mut where_sql: Vec<String> = Vec::default();

    for clause in wheres {
        if let Some(args) = args {
            clause.bind(args);
        }
        if let Some(p) = clause.clause(&alias, &next_params) {
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
        if let Some(p) = clause.clause(&alias, &next_params) {
            where_sql.push(p);
        }
    }

    if where_sql.len() == 0 {
        return None;
    }
    Some(format!("WHERE ( {} )", where_sql.join(" AND ")))
}

fn build_tail<'schema, T, DB>(select: &SelectBuilder<'schema, T, DB>) -> Option<String>
where
    T: HasSchema,
    DB: sqlx::Database + DbLimitSkipWriter,
    <T as HasSchema>::Schema: TableInfo + TableColumns<DB>,
{
    let w = LimitSkipWriter::new::<DB>();
    let mut parts = VecDeque::default();

    if let Some(skiplimit) = w.skiplimit(select.offset, select.limit) {
        parts.push_back(skiplimit);
    }

    // If we are limiting but no order is given force an order (needed for MSSQL)
    if !parts.is_empty() && select.orderby.is_empty() {
        parts.push_front("ORDER BY 1".to_owned())
    }

    if !select.orderby.is_empty() {
        parts.push_front(orderby::to_sql(&select.orderby));
    }

    if parts.is_empty() {
        return None;
    }
    let parts: Vec<String> = parts.drain(..).collect();
    Some(parts.join(" "))
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
        .map(|col| writer.write_with_prefix(&tablealias, col))
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
