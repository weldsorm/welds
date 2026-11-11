use super::RelationshipKeyPairs;
use std::sync::Arc;

/// Used to invert a RelationshipKeyPairs
/// helpful when writing different kinds of clauses
#[derive(Clone)]
pub(crate) struct RelationshipKeyPairsInvert<Inner>
where
    Inner: RelationshipKeyPairs + Send + Sync,
{
    inner: Arc<Inner>,
}

impl<Inner> RelationshipKeyPairsInvert<Inner>
where
    Inner: RelationshipKeyPairs + Send + Sync,
{
    pub fn new(inner: Inner) -> Self {
        Self {
            inner: Arc::new(inner),
        }
    }
}

impl<Inner> RelationshipKeyPairs for RelationshipKeyPairsInvert<Inner>
where
    Inner: RelationshipKeyPairs + Send + Sync,
{
    fn a_key_column(&self) -> String {
        self.inner.b_key_column()
    }
    fn b_key_column(&self) -> String {
        self.inner.a_key_column()
    }
}
