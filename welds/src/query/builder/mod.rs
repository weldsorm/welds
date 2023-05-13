use crate::alias::TableAlias;
use crate::query::clause::exists::ExistIn;
use crate::query::clause::{AsFieldName, ClauseAdder, OrderBy};
use crate::relations::{HasRelations, Relationship};
use crate::table::{HasSchema, TableColumns, TableInfo, UniqueIdentifier};
use crate::writers::column::DbColumnWriter;
use crate::writers::limit_skip::DbLimitSkipWriter;
use std::marker::PhantomData;
use std::rc::Rc;

use super::select_cols::SelectBuilder;
use super::update::bulk::UpdateBuilder;

/// An un-executed Query.
///
/// Build out a query that can be executed on the database.
///
/// Can be chained with other queries to make more complex queries.
///
/// Can be mapped into other queries to  make more complex queries.
pub struct QueryBuilder<'schema, T, DB: sqlx::Database> {
    _t: PhantomData<T>,
    pub(crate) wheres: Vec<Box<dyn ClauseAdder<'schema, DB>>>,
    pub(crate) exist_ins: Vec<ExistIn<'schema, DB>>,
    pub(crate) limit: Option<i64>,
    pub(crate) offset: Option<i64>,
    pub(crate) orderby: Vec<OrderBy>,
    pub(crate) alias: String,
    pub(crate) alias_asigner: Rc<TableAlias>,
}

impl<'schema, T, DB> Default for QueryBuilder<'schema, T, DB>
where
    DB: sqlx::Database,
    T: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row> + HasSchema,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'schema, T, DB> QueryBuilder<'schema, T, DB>
where
    DB: sqlx::Database,
    T: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row> + HasSchema,
{
    pub fn new() -> Self {
        let ta = TableAlias::new();
        let alias = ta.next();
        Self {
            _t: Default::default(),
            wheres: Vec::default(),
            limit: None,
            offset: None,
            orderby: Vec::default(),
            exist_ins: Default::default(),
            alias,
            alias_asigner: Rc::new(ta),
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
        filter: QueryBuilder<'schema, R, DB>,
    ) -> Self
    where
        DB: sqlx::Database + DbLimitSkipWriter,
        T: HasRelations,
        Ship: Relationship<R>,
        R: HasSchema,
        T: HasSchema,
        R: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row> + HasSchema,
        <R as HasSchema>::Schema: TableInfo + TableColumns<DB> + UniqueIdentifier<DB>,
        <T as HasSchema>::Schema: TableInfo + TableColumns<DB> + UniqueIdentifier<DB>,
        <T as HasRelations>::Relation: Default,
    {
        let ship = relationship(Default::default());
        let out_col = ship.my_key::<DB, R::Schema, T::Schema>();
        let inner_tn = <R as HasSchema>::Schema::identifier();
        let inner_tn = inner_tn.join(".");
        let inner_col = ship.their_key::<DB, R::Schema, T::Schema>();
        let mut exist_in = ExistIn::<'schema, DB>::new(filter, out_col, inner_tn, inner_col);
        exist_in.set_aliases(&self.alias_asigner);
        self.exist_ins.push(exist_in);
        self
    }

    /// Results in a query that is mapped into the query of one of its relationships
    pub fn map_query<R, Ship>(
        self,
        relationship: impl Fn(<T as HasRelations>::Relation) -> Ship,
    ) -> QueryBuilder<'schema, R, DB>
    where
        DB: sqlx::Database + DbLimitSkipWriter,
        T: HasRelations,
        Ship: Relationship<R>,
        R: HasSchema,
        T: HasSchema,
        R: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row> + HasSchema,
        <R as HasSchema>::Schema: TableInfo + TableColumns<DB> + UniqueIdentifier<DB>,
        <T as HasSchema>::Schema: TableInfo + TableColumns<DB> + UniqueIdentifier<DB>,
        <T as HasRelations>::Relation: Default,
    {
        let ship = relationship(Default::default());
        let mut qb: QueryBuilder<R, DB> = QueryBuilder::new();
        qb.set_aliases(&self.alias_asigner);

        let out_col = ship.their_key::<DB, R::Schema, T::Schema>();
        let inner_tn = <T as HasSchema>::Schema::identifier().join(".");
        let inner_col = ship.my_key::<DB, R::Schema, T::Schema>();
        let exist_in = ExistIn::<'schema, DB>::new(self, out_col, inner_tn, inner_col);

        qb.exist_ins.push(exist_in);
        qb
    }

    pub(crate) fn set_aliases(&mut self, alias_asigner: &Rc<TableAlias>)
    where
        DB: sqlx::Database + DbLimitSkipWriter,
    {
        self.alias_asigner = alias_asigner.clone();
        self.alias = self.alias_asigner.next();
        for sub in &mut self.exist_ins {
            sub.set_aliases(&self.alias_asigner);
        }
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
    pub fn order_by_desc<V, FN: AsFieldName<V>>(
        mut self,
        lam: impl Fn(<T as HasSchema>::Schema) -> FN,
    ) -> Self {
        let field = lam(Default::default());
        let colname = field.colname();
        self.orderby.push(OrderBy::new(colname, "DESC"));
        self
    }

    /// Order the results of the query by a given column
    ///
    /// multiple calls will result in multiple OrderBys
    pub fn order_by_asc<V, FN: AsFieldName<V>>(
        mut self,
        lam: impl Fn(<T as HasSchema>::Schema) -> FN,
    ) -> Self {
        let field = lam(Default::default());
        let colname = field.colname();
        self.orderby.push(OrderBy::new(colname, "ASC"));
        self
    }

    /// Select only the specific columns
    pub fn select<V, FN: AsFieldName<V>>(
        self,
        lam: impl Fn(<T as HasSchema>::Schema) -> FN,
    ) -> SelectBuilder<'schema, T, DB> {
        let sb = SelectBuilder::new(self);
        sb.select(lam)
    }

    /// Filter the results returned by this query.
    /// Used when you want to filter on the columns of this table.
    pub fn set<'v, V, FIELD>(
        self,
        lam: impl Fn(<T as HasSchema>::Schema) -> FIELD,
        value: impl Into<V>,
    ) -> UpdateBuilder<'schema, T, DB>
    where
        DB: DbColumnWriter,
        <T as HasSchema>::Schema: Default,
        FIELD: AsFieldName<V>,
        V: for<'r> sqlx::Encode<'r, DB> + sqlx::Type<DB> + Send + Clone,
        V: 'static,
    {
        let ub = UpdateBuilder::new(self);
        ub.set(lam, value)
    }
}
