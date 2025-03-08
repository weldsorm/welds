use crate::model_traits::{HasSchema, TableColumns, TableInfo, UniqueIdentifier};
use crate::query::builder::QueryBuilder;
use crate::query::clause::{AsFieldName, ClauseAdder};
use crate::relations::{HasRelations, Relationship};

mod exec;
#[cfg(test)]
mod tests;

/// An un-executed Query Selecting a model AND its relationship objects.
pub struct IncludeBuilder<T> {
    qb: QueryBuilder<T>,
}

impl<T> IncludeBuilder<T>
where
    T: Send + HasSchema,
{
    pub fn new(qb: QueryBuilder<T>) -> Self {
        Self { qb }
    }

    /// Include other related objects in a returned Dataset
    pub fn include<R, Ship>(
        self,
        relationship: impl Fn(<T as HasRelations>::Relation) -> Ship,
    ) -> IncludeBuilder<T>
    where
        T: HasRelations,
        Ship: Relationship<R>,
        R: HasSchema,
        R: Send + Sync + HasSchema,
        <R as HasSchema>::Schema: TableInfo + TableColumns + UniqueIdentifier,
        <T as HasSchema>::Schema: TableInfo + TableColumns + UniqueIdentifier,
        <T as HasRelations>::Relation: Default,
    {
        todo!()
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

    // pub(crate) fn set_aliases(&mut self, alias_asigner: &Arc<TableAlias>) {
    //     self.qb.set_aliases(alias_asigner);
    //     for join in &mut self.joins {
    //         join.set_aliases(&self.qb.alias_asigner);
    //     }
    // }
}
