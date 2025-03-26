use crate::connections::Row;
use crate::errors::Result;
use crate::errors::WeldsError;
use crate::exts::VecStateExt;
use crate::model_traits::{HasSchema, TableColumns, TableInfo, UniqueIdentifier};
use crate::query::builder::QueryBuilder;
use crate::query::clause::exists::ExistIn;
use crate::relations::Relationship;
use crate::Client;
use async_trait::async_trait;
use std::any::Any;

#[async_trait]
pub(crate) trait RelatedQuery<R> {
    async fn run(
        &self,
        primary_query: &QueryBuilder<R>,
        client: &dyn Client,
    ) -> Result<Box<dyn RelatedSetAccesser + Send>>;
    fn to_sql(&self, primary_query: &QueryBuilder<R>, syntax: crate::Syntax) -> String;
}

pub(crate) struct IncludeQuery<R, Ship>
where
    Ship: Relationship<R>,
{
    // The model that is being included
    pub(crate) row_type: std::marker::PhantomData<R>,
    pub(crate) out_col: String,
    pub(crate) inner_tn: String,
    pub(crate) inner_col: String,
    pub(crate) ship: Ship,
}

#[async_trait]
impl<R, T, Ship> RelatedQuery<T> for IncludeQuery<R, Ship>
where
    for<'r> &'r IncludeQuery<R, Ship>: Send,
    for<'b> &'b QueryBuilder<T>: Send,
    Ship: 'static + Relationship<R>,
    T: 'static + Send,
    R: 'static,
    <R as HasSchema>::Schema: TableInfo + TableColumns + UniqueIdentifier,
    R: Send + Sync + HasSchema,
    R: TryFrom<Row>,
    WeldsError: From<<R as TryFrom<Row>>::Error>,
{
    async fn run(
        &self,
        primary_query: &QueryBuilder<T>,
        client: &dyn Client,
    ) -> Result<Box<dyn RelatedSetAccesser + Send>> {
        let primary_query = primary_query.clone();

        let mut qb: QueryBuilder<R> = QueryBuilder::new();
        qb.set_aliases(&primary_query.alias_asigner);

        let exist_in = ExistIn::new(
            &primary_query,
            self.out_col.clone(),
            self.inner_tn.clone(),
            self.inner_col.clone(),
        );
        qb.exist_ins.push(exist_in);
        let rows = qb.run(client).await?;

        Ok(Box::new(RelatedSet::<R, Ship> {
            data: rows.into_inners(),
            ship: self.ship.clone(),
        }))
    }

    fn to_sql(&self, primary_query: &QueryBuilder<T>, syntax: crate::Syntax) -> String {
        let primary_query = primary_query.clone();
        let mut qb: QueryBuilder<R> = QueryBuilder::new();
        qb.set_aliases(&primary_query.alias_asigner);
        let exist_in = ExistIn::new(
            &primary_query,
            self.out_col.clone(),
            self.inner_tn.clone(),
            self.inner_col.clone(),
        );
        qb.exist_ins.push(exist_in);
        qb.to_sql(syntax)
    }
}

pub(crate) struct RelatedSet<R, Ship>
where
    Ship: Relationship<R>,
{
    pub(crate) data: Vec<R>,
    pub(crate) ship: Ship,
}

pub(crate) trait RelatedSetAccesser {
    fn as_any(&self) -> &dyn Any;
    //fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<R: 'static, Ship: 'static> RelatedSetAccesser for RelatedSet<R, Ship>
where
    Ship: Relationship<R>,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    //fn as_any_mut(&mut self) -> &mut dyn Any {
    //    self
    //}
}

pub(crate) trait SetDowncast {
    fn downcast_ref<R: 'static, Ship: 'static + Relationship<R>>(
        &self,
    ) -> Option<&RelatedSet<R, Ship>>;
    // fn downcast_mut<R: 'static, Ship: 'static + Relationship<R>>(
    //     &mut self,
    // ) -> Option<&mut RelatedSet<R, Ship>>;
}

impl SetDowncast for Box<dyn RelatedSetAccesser + Send> {
    fn downcast_ref<R: 'static, Ship: 'static + Relationship<R>>(
        &self,
    ) -> Option<&RelatedSet<R, Ship>> {
        self.as_any().downcast_ref::<RelatedSet<R, Ship>>()
    }

    // fn downcast_mut<R: 'static, Ship: 'static + Relationship<R>>(
    //     &mut self,
    // ) -> Option<&mut RelatedSet<R, Ship>> {
    //     self.as_any_mut().downcast_mut::<RelatedSet<R, Ship>>()
    // }
}
