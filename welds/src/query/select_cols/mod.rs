use crate::model_traits::{HasSchema, TableColumns, TableInfo, UniqueIdentifier};
use crate::query::builder::QueryBuilder;
use crate::query::clause::{AsFieldName, ClauseAdder};
use crate::query::select_cols::select_column::SelectKind;
use crate::query::select_cols::group_by::GroupBy;
use crate::relations::{HasRelations, Relationship};
use crate::writers::alias::TableAlias;
pub use join::Join;
use join::JoinBuilder;
use select_column::SelectColumn;
use std::sync::Arc;

mod exec;
mod join;
mod select_column;
mod group_by;

#[cfg(test)]
mod tests;

/// An un-executed Query Selecting specific columns.
///
/// Build out a query that can be executed on the database.
///
/// Can be chained with other queries to make more complex queries.
///
/// Can be mapped into other queries to make more complex queries.
pub struct SelectBuilder<T> {
    qb: QueryBuilder<T>,
    selects: Vec<SelectColumn>,
    joins: Vec<JoinBuilder>,
    group_bys: Vec<GroupBy>,
}

impl<T> SelectBuilder<T>
where
    T: Send + HasSchema,
{
    pub fn new(qb: QueryBuilder<T>) -> Self {
        Self {
            qb,
            selects: Vec::default(),
            joins: Vec::default(),
            group_bys: Vec::default(),
        }
    }

    /// Add a columns to the specific list of columns that will be selected
    pub fn select<V, FN: AsFieldName<V>>(
        mut self,
        lam: impl Fn(<T as HasSchema>::Schema) -> FN,
    ) -> SelectBuilder<T> {
        let field = lam(Default::default());
        self.selects.push(SelectColumn {
            col_name: field.colname().to_string(),
            field_name: field.fieldname().to_string(),
            kind: SelectKind::Column,
        });
        self
    }

    /// Add a columns to the specific list of columns that will be selected
    /// uses a sql "AS" to rename the returns column so it can match
    /// the struct you are selecting into
    pub fn select_as<V, FN: AsFieldName<V>>(
        mut self,
        lam: impl Fn(<T as HasSchema>::Schema) -> FN,
        as_name: &'static str,
    ) -> SelectBuilder<T> {
        let field = lam(Default::default());
        self.selects.push(SelectColumn {
            col_name: field.colname().to_string(),
            field_name: as_name.to_string(),
            kind: SelectKind::Column,
        });
        self
    }

    pub fn select_count<V, FN: AsFieldName<V>>(
        mut self,
        lam: impl Fn(<T as HasSchema>::Schema) -> FN,
        as_name: &'static str,
    ) -> SelectBuilder<T> {
        let field = lam(Default::default());
        self.selects.push(SelectColumn {
            col_name: field.colname().to_string(),
            field_name: as_name.to_string(),
            kind: SelectKind::Count,
        });
        self
    }

    pub fn select_max<V, FN: AsFieldName<V>>(
        mut self,
        lam: impl Fn(<T as HasSchema>::Schema) -> FN,
        as_name: &'static str,
    ) -> SelectBuilder<T> {
        let field = lam(Default::default());
        self.selects.push(SelectColumn {
            col_name: field.colname().to_string(),
            field_name: as_name.to_string(),
            kind: SelectKind::Max,
        });
        self
    }

    pub fn select_min<V, FN: AsFieldName<V>>(
        mut self,
        lam: impl Fn(<T as HasSchema>::Schema) -> FN,
        as_name: &'static str,
    ) -> SelectBuilder<T> {
        let field = lam(Default::default());
        self.selects.push(SelectColumn {
            col_name: field.colname().to_string(),
            field_name: as_name.to_string(),
            kind: SelectKind::Min,
        });
        self
    }

    /// Filter the results returned by this query.
    /// Used when you want to filter on the columns of this table.
    pub fn where_col(
        mut self,
        lam: impl Fn(<T as HasSchema>::Schema) -> Box<dyn ClauseAdder>,
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
        filter: QueryBuilder<R>,
    ) -> Self
    where
        T: HasRelations,
        Ship: Relationship<R>,
        R: HasSchema,
        T: HasSchema,
        R: Send + HasSchema,
        <R as HasSchema>::Schema: TableInfo + TableColumns + UniqueIdentifier,
        <T as HasSchema>::Schema: TableInfo + TableColumns + UniqueIdentifier,
        <T as HasRelations>::Relation: Default,
    {
        self.qb = self.qb.where_relation(relationship, filter);
        self
    }

    /// Inner Join to another table to be able to select additional columns
    pub fn join<R, Ship>(
        self,
        relationship: impl Fn(<T as HasRelations>::Relation) -> Ship,
        sb: SelectBuilder<R>,
    ) -> Self
    where
        T: HasRelations,
        Ship: Relationship<R>,
        R: HasSchema,
        R: Send + HasSchema,
        <R as HasSchema>::Schema: TableInfo + TableColumns + UniqueIdentifier,
        <T as HasSchema>::Schema: TableInfo + TableColumns + UniqueIdentifier,
    {
        self.join_with(relationship, sb, Join::Inner)
    }

    /// left Join to another table to be able to select additional columns
    pub fn left_join<R, Ship>(
        self,
        relationship: impl Fn(<T as HasRelations>::Relation) -> Ship,
        sb: SelectBuilder<R>,
    ) -> Self
    where
        T: HasRelations,
        Ship: Relationship<R>,
        R: HasSchema,
        R: Send + HasSchema,
        <R as HasSchema>::Schema: TableInfo + TableColumns + UniqueIdentifier,
        <T as HasSchema>::Schema: TableInfo + TableColumns + UniqueIdentifier,
    {
        self.join_with(relationship, sb, Join::Left)
    }

    /// Join to another table to be able to select additional columns
    /// Allow manual selection of Join Type
    pub fn join_with<R, Ship>(
        mut self,
        relationship: impl Fn(<T as HasRelations>::Relation) -> Ship,
        mut sb: SelectBuilder<R>,
        join_type: Join,
    ) -> Self
    where
        T: HasRelations,
        Ship: Relationship<R>,
        R: HasSchema,
        R: Send + HasSchema,
        <R as HasSchema>::Schema: TableInfo + TableColumns + UniqueIdentifier,
        <T as HasSchema>::Schema: TableInfo + TableColumns + UniqueIdentifier,
    {
        let ship = relationship(Default::default());
        sb.set_aliases(&self.qb.alias_asigner);
        let outer_key = ship.my_key::<R::Schema, T::Schema>();
        let inner_key = ship.their_key::<R::Schema, T::Schema>();
        let mut jb = JoinBuilder::new(sb, outer_key, inner_key);
        jb.ty = join_type;
        self.joins.push(jb);
        self
    }

    pub fn group_by<V, FN: AsFieldName<V>>(
        mut self,
        lam: impl Fn(<T as HasSchema>::Schema) -> FN,
    ) -> Self {
        let field = lam(Default::default());
        self.group_bys.push(GroupBy {
            col_name: field.colname().to_string(),
        });
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

    pub(crate) fn set_aliases(&mut self, alias_asigner: &Arc<TableAlias>) {
        self.qb.set_aliases(alias_asigner);
        for join in &mut self.joins {
            join.set_aliases(&self.qb.alias_asigner);
        }
    }
}
