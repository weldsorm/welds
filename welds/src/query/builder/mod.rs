use super::clause::{self, AsOptField};
use super::select_cols::SelectBuilder;
pub use super::update::bulk::UpdateBuilder;
use crate::model_traits::{HasSchema, TableColumns, TableInfo, UniqueIdentifier};
use crate::query::clause::exists::ExistIn;
use crate::query::clause::{AsFieldName, AssignmentAdder, ClauseAdder, OrderBy};
use crate::query::include::IncludeBuilder;
use crate::relations::{HasRelations, Relationship};
use crate::writers::alias::TableAlias;
use std::marker::PhantomData;
use std::sync::Arc;
use welds_connections::Param;

pub use super::clause::manualparam::ManualParam;

#[deprecated(note = "please use `ManualParam` instead")]
pub type ManualWhereParam = ManualParam;

#[cfg(test)]
mod tests;

/// An un-executed Query.
///
/// Build out a query that can be executed on the database.
///
/// Can be chained with other queries to make more complex queries.
///
/// Can be mapped into other queries to  make more complex queries.
pub struct QueryBuilder<T> {
    _t: PhantomData<T>,
    pub(crate) wheres: Vec<Arc<Box<dyn ClauseAdder>>>,
    pub(crate) exist_ins: Vec<ExistIn>,
    pub(crate) limit: Option<i64>,
    pub(crate) offset: Option<i64>,
    pub(crate) orderby: Vec<OrderBy>,
    pub(crate) alias: String,
    pub(crate) alias_asigner: Arc<TableAlias>,
}

impl<T> Clone for QueryBuilder<T> {
    fn clone(&self) -> Self {
        Self {
            _t: Default::default(),
            wheres: self.wheres.clone(),
            limit: self.limit,
            offset: self.offset,
            orderby: self.orderby.clone(),
            exist_ins: self.exist_ins.clone(),
            alias: self.alias.clone(),
            alias_asigner: self.alias_asigner.clone(),
        }
    }
}

impl<T> Default for QueryBuilder<T>
where
    T: Send + HasSchema,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> QueryBuilder<T>
where
    T: Send + HasSchema,
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
            alias_asigner: Arc::new(ta),
        }
    }

    /// Filter the results returned by this query.
    /// Used when you want to filter on the columns of this table.
    /// This is the default way to write `WHERE` clauses
    /// ```
    /// use welds::prelude::*;
    ///
    /// #[derive(Debug, Default, WeldsModel)]
    /// #[welds(table = "thing")]
    /// struct Thing {
    ///     #[welds(primary_key)]
    ///     pub id: i32,
    /// }
    ///
    /// async fn example(db: &dyn Client) -> welds::errors::Result<()> {
    ///     let rows = Thing::all().where_col(|c| c.id.gt(10) ).run(db).await?;
    ///     Ok(())
    /// }
    /// ```
    pub fn where_col(
        mut self,
        lam: impl Fn(<T as HasSchema>::Schema) -> Box<dyn ClauseAdder>,
    ) -> Self
    where
        <T as HasSchema>::Schema: Default,
    {
        let qba = lam(Default::default());
        self.wheres.push(Arc::new(qba));
        self
    }

    /// write custom sql for the right side of a clause in a where block
    ///
    /// NOTE: use '?' for params. They will be swapped out for the correct Syntax
    ///
    /// NOTE: use '$' for table prefix/alias. It will be swapped out for the prefix used at runtime
    ///
    /// Example
    /// ```
    /// use welds::prelude::*;
    /// use welds::query::builder::ManualParam;
    ///
    /// #[derive(Debug, Default, WeldsModel)]
    /// #[welds(table = "thing")]
    /// struct Thing {
    ///     #[welds(primary_key)]
    ///     pub id: i32,
    ///     pub price1: i32,
    ///     pub price2: i32,
    /// }
    ///
    /// async fn example(db: &dyn Client) -> welds::errors::Result<()> {
    ///     let params = ManualParam::new().push(5);
    ///     let rows = Thing::all().where_manual(|c| c.price1, " > $.price2 + ?", params).run(db).await?;
    ///     // will result in:
    ///     // WHERE t1.price1 > t1.price2 + 5
    ///     Ok(())
    /// }
    /// ```
    ///
    pub fn where_manual<V, FN>(
        mut self,
        col: impl Fn(<T as HasSchema>::Schema) -> FN,
        sql: &'static str,
        params: impl Into<ManualParam>,
    ) -> Self
    where
        FN: AsFieldName<V>,
    {
        let field = col(Default::default());
        let colname = field.colname().to_string();
        let params: ManualParam = params.into();
        let c = clause::ClauseColManual {
            col: Some(colname),
            sql: sql.to_string(),
            params: params.into_inner(),
        };
        self.wheres.push(Arc::new(Box::new(c)));
        self
    }

    /// write custom sql for a clause in a where block
    ///
    /// NOTE: use '?' for params. They will be swapped out for the correct Syntax
    ///
    /// NOTE: use '$' for table prefix/alias. It will be swapped out for the prefix used at runtime
    ///
    /// Example
    /// ```
    /// use welds::prelude::*;
    /// use welds::query::builder::ManualParam;
    ///
    /// #[derive(Debug, Default, WeldsModel)]
    /// #[welds(table = "thing")]
    /// struct Thing {
    ///     #[welds(primary_key)]
    ///     pub id: i32,
    ///     pub price1: i32,
    ///     pub price2: i32,
    /// }
    ///
    /// async fn example(db: &dyn Client) -> welds::errors::Result<()> {
    ///     let params = ManualParam::new().push(5);
    ///     let rows = Thing::all().where_manual2("$.price1 + $.price2 > ?", params).run(db).await?;
    ///     // will result in:
    ///     // WHERE t1.price1 + t1.price2 > 5
    ///     Ok(())
    /// }
    /// ```
    ///
    pub fn where_manual2(mut self, sql: &'static str, params: impl Into<ManualParam>) -> Self {
        let params: ManualParam = params.into();
        let c = clause::ClauseColManual {
            col: None,
            sql: sql.to_string(),
            params: params.into_inner(),
        };
        self.wheres.push(Arc::new(Box::new(c)));
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
        R: Send + Sync + HasSchema,
        <R as HasSchema>::Schema: TableInfo + TableColumns + UniqueIdentifier,
        <T as HasSchema>::Schema: TableInfo + TableColumns + UniqueIdentifier,
        <T as HasRelations>::Relation: Default,
    {
        let ship = relationship(Default::default());
        let out_col = ship.my_key::<R::Schema, T::Schema>();
        let inner_tn = <R as HasSchema>::Schema::identifier();
        let inner_tn = inner_tn.join(".");
        let inner_col = ship.their_key::<R::Schema, T::Schema>();
        let mut exist_in = ExistIn::new(&filter, out_col, inner_tn, inner_col);
        exist_in.set_aliases(&self.alias_asigner);
        self.exist_ins.push(exist_in);
        self
    }

    /// Results in a query that is mapped into the query of one of its relationships
    pub fn map_query<R, Ship>(
        &self,
        relationship: impl Fn(<T as HasRelations>::Relation) -> Ship,
    ) -> QueryBuilder<R>
    where
        T: HasRelations,
        Ship: Relationship<R>,
        T: HasSchema,
        R: Send + Sync + HasSchema,
        <R as HasSchema>::Schema: TableInfo + TableColumns + UniqueIdentifier,
        <T as HasSchema>::Schema: TableInfo + TableColumns + UniqueIdentifier,
        <T as HasRelations>::Relation: Default,
    {
        let ship = relationship(Default::default());
        let mut qb: QueryBuilder<R> = QueryBuilder::new();
        qb.set_aliases(&self.alias_asigner);

        let out_col = ship.their_key::<R::Schema, T::Schema>();
        let inner_tn = <T as HasSchema>::Schema::identifier().join(".");
        let inner_col = ship.my_key::<R::Schema, T::Schema>();
        let exist_in = ExistIn::new(self, out_col, inner_tn, inner_col);

        qb.exist_ins.push(exist_in);
        qb
    }

    pub(crate) fn set_aliases(&mut self, alias_asigner: &Arc<TableAlias>) {
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
    /// puts NULLs as the end of the resulting rows
    ///
    /// multiple calls will result in multiple OrderBys
    pub fn order_by_desc_null_last<V, FN: AsFieldName<V>>(
        mut self,
        lam: impl Fn(<T as HasSchema>::Schema) -> FN,
    ) -> Self {
        let field = lam(Default::default());
        let colname = field.colname();
        let colnull = format!("{colname} is null");
        self.orderby.push(OrderBy::new(colnull, "ASC"));
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

    /// Order the results of the query by a given column
    /// puts NULLs at the front of the resulting rows
    ///
    /// multiple calls will result in multiple OrderBys
    pub fn order_by_asc_null_first<V, FN: AsFieldName<V>>(
        mut self,
        lam: impl Fn(<T as HasSchema>::Schema) -> FN,
    ) -> Self {
        let field = lam(Default::default());
        let colname = field.colname();
        let colnull = format!("{colname} is null");
        self.orderby.push(OrderBy::new(colnull, "DESC"));
        self.orderby.push(OrderBy::new(colname, "ASC"));
        self
    }

    /// Manually write the order by part of the query
    pub fn order_manual(mut self, sql: &str) -> Self {
        self.orderby.push(OrderBy::new(sql.to_string(), ""));
        self
    }

    /// Select only the specific columns
    pub fn select<V, FN: AsFieldName<V>>(
        self,
        lam: impl Fn(<T as HasSchema>::Schema) -> FN,
    ) -> SelectBuilder<T> {
        let sb = SelectBuilder::new(self);
        sb.select(lam)
    }

    /// Select only the specific columns
    /// uses a sql "AS" to rename the selected column so it can match
    /// the struct you are selecting into
    pub fn select_as<V, FN: AsFieldName<V>>(
        self,
        lam: impl Fn(<T as HasSchema>::Schema) -> FN,
        as_name: &'static str,
    ) -> SelectBuilder<T> {
        let sb = SelectBuilder::new(self);
        sb.select_as(lam, as_name)
    }

    /// Changes this query Into a sql UPDATE.
    /// Sets the value from the lambda in the database
    ///
    /// ```
    /// use welds::prelude::*;
    ///
    /// #[derive(Debug, Default, WeldsModel)]
    /// #[welds(table = "things")]
    /// struct Thing {
    ///     #[welds(primary_key)]
    ///     pub id: i32,
    ///     pub foo: i32,
    /// }
    ///
    /// async fn example(db: &dyn Client) -> welds::errors::Result<()> {
    ///     Thing::all().set(|x| x.foo, 42).run(db).await?;
    ///     // [UPDATE things SET foo = ?]   (?=42)
    ///     Ok(())
    /// }
    ///
    pub fn set<V, FIELD>(
        self,
        lam: impl Fn(<T as HasSchema>::Schema) -> FIELD,
        value: impl Into<V>,
    ) -> UpdateBuilder<T>
    where
        <T as HasSchema>::Schema: Default,
        FIELD: AsFieldName<V>,
        V: 'static + Sync + Send + Clone + Param,
    {
        let ub = UpdateBuilder::new(self);
        ub.set(lam, value)
    }

    /// Changes this query Into a sql UPDATE.
    /// Sets a value from the lambda into the database
    ///
    /// ```
    /// use welds::prelude::*;
    ///
    /// #[derive(Debug, Default, WeldsModel)]
    /// #[welds(table = "thing")]
    /// struct Thing {
    ///     #[welds(primary_key)]
    ///     pub id: i32,
    ///     pub foo: i32,
    /// }
    ///
    /// async fn example(db: &dyn Client) -> welds::errors::Result<()> {
    ///     Thing::all().set_col(|x| x.foo.equal(42) ).run(db).await?;
    ///     Ok(())
    /// }
    ///
    /// ```
    ///
    pub fn set_col(
        self,
        lam: impl Fn(<T as HasSchema>::Schema) -> Box<dyn AssignmentAdder>,
    ) -> UpdateBuilder<T>
    where
        <T as HasSchema>::Schema: Default,
    {
        let ub = UpdateBuilder::new(self);
        ub.set_col(lam)
    }

    /// Nulls out the value from the lambda in the database
    pub fn set_null<V, FIELD>(
        self,
        lam: impl Fn(<T as HasSchema>::Schema) -> FIELD,
    ) -> UpdateBuilder<T>
    where
        <T as HasSchema>::Schema: Default,
        FIELD: AsFieldName<V> + AsOptField,
        V: 'static + Sync + Send + Clone + Param,
    {
        let ub = UpdateBuilder::new(self);
        ub.set_null(lam)
    }

    /// Write custom sql for the right side of a SET clause
    ///
    /// NOTE: use '?' for params. They will be swapped out for the correct Syntax
    ///
    /// ```
    /// use welds::prelude::*;
    /// use welds::query::builder::ManualParam;
    ///
    /// #[derive(Debug, Default, WeldsModel)]
    /// #[welds(table = "things")]
    /// struct Thing {
    ///     #[welds(primary_key)]
    ///     pub id: i32,
    ///     pub num: i32,
    /// }
    ///
    /// async fn example(db: &dyn Client) -> welds::errors::Result<()> {
    ///     let params = ManualParam::new().push(42);
    ///     Thing::all().set_manual(|x| x.num, "num+?", params).run(db).await?;
    ///     // [UPDATE things SET num = (num+?)]   (?=42)
    ///     Ok(())
    /// }
    ///
    pub fn set_manual<V, FIELD>(
        self,
        lam: impl Fn(<T as HasSchema>::Schema) -> FIELD,
        sql: &'static str,
        params: impl Into<ManualParam>,
    ) -> UpdateBuilder<T>
    where
        <T as HasSchema>::Schema: Default,
        FIELD: AsFieldName<V>,
        V: 'static + Sync + Send + Clone + Param,
    {
        let ub = UpdateBuilder::new(self);
        ub.set_manual(lam, sql, params)
    }

    /// Include an other related to this one. `BelongsTo` `HasMany`.
    /// querying will continue over your current Object, but the related object will be
    /// accessible in the resulting dataset off of each instance of your model
    pub fn include<R, Ship>(
        self,
        relationship: impl Fn(<T as HasRelations>::Relation) -> Ship,
    ) -> IncludeBuilder<T>
    where
        T: 'static + HasRelations,
        Ship: 'static + Sync + Relationship<R>,
        R: HasSchema,
        R: 'static,
        R: Send + Sync + HasSchema,
        <R as HasSchema>::Schema: TableInfo + TableColumns + UniqueIdentifier,
        <T as HasSchema>::Schema: TableInfo + TableColumns + UniqueIdentifier,
        <T as HasRelations>::Relation: Default,
        R: TryFrom<crate::connections::Row>,
        crate::errors::WeldsError: From<<R as TryFrom<crate::connections::Row>>::Error>,
    {
        let ib = IncludeBuilder::new(self);
        ib.include(relationship)
    }
}
