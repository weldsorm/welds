use crate::dataset::DataSet;
use crate::errors::Result;
use crate::errors::WeldsError;
use crate::model_traits::{HasSchema, TableColumns, TableInfo};
use crate::query::builder::QueryBuilder;
use crate::query::include::IncludeBuilder;
use crate::Client;
use crate::Row;
use crate::Syntax;
use async_trait::async_trait;

#[async_trait]
pub(crate) trait RelatedQuery<T> {
    async fn run(&self, qb: &QueryBuilder<T>) -> Result<Box<dyn RelatedSet>>;
}

pub(crate) trait RelatedSet {}

impl

impl<T> TryFrom<&Box<dyn RelatedSet>> for &[T] {
    type Error = ();
    fn try_from(_value: &Box<dyn RelatedSet>) -> std::result::Result<Self, Self::Error> {
        Err(())
    }
}
