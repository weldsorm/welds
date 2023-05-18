use crate::alias::TableAlias;
use crate::query::builder::QueryBuilder;
use crate::query::clause::{AsFieldName, ClauseAdder};
use crate::relations::{HasRelations, Relationship};
use crate::table::{HasSchema, TableColumns, TableInfo, UniqueIdentifier};
use crate::writers::limit_skip::DbLimitSkipWriter;
pub use join::Join;
use join::JoinBuilder;
use select_column::SelectColumn;
use std::rc::Rc;

mod exec;
mod join;
mod select_column;

/// An un-executed Query Selecting specific columns.
///
/// Build out a query that can be executed on the database.
///
/// Can be chained with other queries to make more complex queries.
///
/// Can be mapped into other queries to make more complex queries.
pub struct SelectBuilder<'schema, T, DB: sqlx::Database> {
    qb: QueryBuilder<'schema, T, DB>,
    selects: Vec<SelectColumn>,
    joins: Vec<JoinBuilder<'schema, DB>>,
}

impl<'schema, T, DB> SelectBuilder<'schema, T, DB>
where
    DB: sqlx::Database,
    T: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row> + HasSchema,
{
    pub fn new(qb: QueryBuilder<'schema, T, DB>) -> Self {
        Self {
            qb,
            selects: Vec::default(),
            joins: Vec::default(),
        }
    }

    /// Add a columns to the specific list of columns that will be selected
    pub fn select<V, FN: AsFieldName<V>>(
        mut self,
        lam: impl Fn(<T as HasSchema>::Schema) -> FN,
    ) -> SelectBuilder<'schema, T, DB> {
        let field = lam(Default::default());
        self.selects.push(SelectColumn {
            col_name: field.colname().to_string(),
            field_name: field.fieldname().to_string(),
        });
        self
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
        self.qb = self.qb.where_col(lam);
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
        self.qb = self.qb.where_relation(relationship, filter);
        self
    }

    /// Inner Join to another table to be able to select additional columns
    pub fn join<R, Ship>(
        self,
        relationship: impl Fn(<T as HasRelations>::Relation) -> Ship,
        sb: SelectBuilder<'schema, R, DB>,
    ) -> Self
    where
        DB: sqlx::Database + DbLimitSkipWriter,
        T: HasRelations,
        Ship: Relationship<R>,
        R: HasSchema,
        R: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row> + HasSchema,
        <R as HasSchema>::Schema: TableInfo + TableColumns<DB> + UniqueIdentifier<DB>,
        <T as HasSchema>::Schema: TableInfo + TableColumns<DB> + UniqueIdentifier<DB>,
    {
        self.join_with(relationship, sb, Join::Inner)
    }

    /// left Join to another table to be able to select additional columns
    pub fn left_join<R, Ship>(
        self,
        relationship: impl Fn(<T as HasRelations>::Relation) -> Ship,
        sb: SelectBuilder<'schema, R, DB>,
    ) -> Self
    where
        DB: sqlx::Database + DbLimitSkipWriter,
        T: HasRelations,
        Ship: Relationship<R>,
        R: HasSchema,
        R: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row> + HasSchema,
        <R as HasSchema>::Schema: TableInfo + TableColumns<DB> + UniqueIdentifier<DB>,
        <T as HasSchema>::Schema: TableInfo + TableColumns<DB> + UniqueIdentifier<DB>,
    {
        self.join_with(relationship, sb, Join::Left)
    }

    /// Join to another table to be able to select additional columns
    /// Allow manual selection of Join Type
    pub fn join_with<R, Ship>(
        mut self,
        relationship: impl Fn(<T as HasRelations>::Relation) -> Ship,
        mut sb: SelectBuilder<'schema, R, DB>,
        join_type: Join,
    ) -> Self
    where
        DB: sqlx::Database + DbLimitSkipWriter,
        T: HasRelations,
        Ship: Relationship<R>,
        R: HasSchema,
        R: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row> + HasSchema,
        <R as HasSchema>::Schema: TableInfo + TableColumns<DB> + UniqueIdentifier<DB>,
        <T as HasSchema>::Schema: TableInfo + TableColumns<DB> + UniqueIdentifier<DB>,
    {
        let ship = relationship(Default::default());
        sb.set_aliases(&self.qb.alias_asigner);
        let outer_key = ship.my_key::<DB, R::Schema, T::Schema>();
        let inner_key = ship.their_key::<DB, R::Schema, T::Schema>();
        let mut jb = JoinBuilder::new(sb, outer_key, inner_key);
        jb.ty = join_type;
        self.joins.push(jb);
        self
    }

    /// Limit the number of rows returned by this query
    pub fn limit(mut self, x: i64) -> Self {
        self.qb = self.qb.limit(x);
        self
    }

    /// Offset the starting point for the results returned by this query
    pub fn offset(mut self, x: i64) -> Self {
        self.qb = self.qb.offset(x);
        self
    }

    /// Order the results of the query by a given column
    ///
    /// multiple calls will result in multiple OrderBys
    pub fn order_by_desc<V, FN: AsFieldName<V>>(
        mut self,
        lam: impl Fn(<T as HasSchema>::Schema) -> FN,
    ) -> Self {
        self.qb = self.qb.order_by_desc(lam);
        self
    }

    /// Order the results of the query by a given column
    ///
    /// multiple calls will result in multiple OrderBys
    pub fn order_by_asc<V, FN: AsFieldName<V>>(
        mut self,
        lam: impl Fn(<T as HasSchema>::Schema) -> FN,
    ) -> Self {
        self.qb = self.qb.order_by_asc(lam);
        self
    }

    pub(crate) fn set_aliases(&mut self, alias_asigner: &Rc<TableAlias>)
    where
        DB: sqlx::Database + DbLimitSkipWriter,
    {
        self.qb.set_aliases(alias_asigner);
        for join in &mut self.joins {
            join.set_aliases(&self.qb.alias_asigner);
        }
    }
}
