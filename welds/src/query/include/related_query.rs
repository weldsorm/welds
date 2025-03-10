use crate::connections::Row;
use crate::errors::Result;
use crate::errors::WeldsError;
use crate::model_traits::{HasSchema, TableColumns, TableInfo, UniqueIdentifier};
use crate::query::builder::QueryBuilder;
use crate::query::clause::exists::ExistIn;
use crate::state::DbState;
use crate::Client;
use async_trait::async_trait;
use std::any::Any;
use std::any::TypeId;

#[async_trait]
pub(crate) trait RelatedQuery<R> {
    async fn run(
        &self,
        primary_query: &QueryBuilder<R>,
        client: &dyn Client,
    ) -> Result<Box<dyn RelatedSetAccesser>>;
}

pub(crate) struct IncludeQuery<T> {
    // The model that is being included
    pub(crate) row_type: std::marker::PhantomData<T>,
    pub(crate) out_col: String,
    pub(crate) inner_tn: String,
    pub(crate) inner_col: String,
}

#[async_trait]
impl<R, T> RelatedQuery<T> for IncludeQuery<R>
where
    for<'r> &'r IncludeQuery<R>: Send,
    for<'b> &'b QueryBuilder<T>: Send,
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
    ) -> Result<Box<dyn RelatedSetAccesser>> {
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

        Ok(Box::new(RelatedSet::<R> { data: rows }))
    }
}

pub(crate) struct RelatedSet<R> {
    data: Vec<DbState<R>>,
}

pub(crate) trait RelatedSetAccesser {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<R: 'static> RelatedSetAccesser for RelatedSet<R> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub(crate) trait SetDowncast {
    fn is<T: 'static>(&self) -> bool;
    fn downcast_ref<R: 'static>(&self) -> Option<&[DbState<R>]>;
    fn downcast_mut<R: 'static>(&mut self) -> Option<&mut [DbState<R>]>;
}

impl SetDowncast for Box<dyn RelatedSetAccesser> {
    fn is<R: 'static>(&self) -> bool {
        // Check if the boxed object is of type T
        self.as_any().type_id() == TypeId::of::<RelatedSet<R>>()
    }

    fn downcast_ref<R: 'static>(&self) -> Option<&[DbState<R>]> {
        if self.is::<R>() {
            let rs: &RelatedSet<R> =
                unsafe { &*(self.as_any() as *const dyn Any as *const RelatedSet<R>) };
            Some(&rs.data)
        } else {
            None
        }
    }

    fn downcast_mut<R: 'static>(&mut self) -> Option<&mut [DbState<R>]> {
        if self.is::<R>() {
            let rs = unsafe { &mut *(self.as_any_mut() as *mut dyn Any as *mut RelatedSet<R>) };
            Some(&mut rs.data)
        } else {
            None
        }
    }
}
