use crate::model_traits::{HasSchema, TableColumns, TableInfo, UniqueIdentifier};
use crate::query::include::RelatedSet;
use crate::state::DbState;
use std::ops::Deref;

#[cfg(test)]
mod tests;

/// A Collection object that hold a set of data that has been
/// selected out of the database and its related objects
pub struct DataSet<T> {
    // not sure if we want to use state or not
    primary: Vec<DbState<T>>,
    related: Vec<Box<dyn RelatedSet>>,
}

impl<T> DataSet<T> {
    pub(crate) fn new(primary: Vec<DbState<T>>, related: Vec<Box<dyn RelatedSet>>) -> Self {
        Self { primary, related }
    }

    fn iter(&self) -> DataSetIter<T> {
        DataSetIter {
            index: 0,
            inner: self,
        }
    }
}

struct DataSetIter<'t, T> {
    index: usize,
    inner: &'t DataSet<T>,
}

impl<'t, T> Iterator for DataSetIter<'t, T> {
    type Item = DataAccessGuard<'t, T>;
    fn next(&mut self) -> Option<Self::Item> {
        let obj = self.inner.primary.get(self.index)?;
        self.index += 1;
        Some(DataAccessGuard {
            inner: obj,
            sets: &self.inner,
        })
    }
}

struct DataAccessGuard<'t, T> {
    inner: &'t T,
    sets: &'t DataSet<T>,
}

impl<'t, T> Deref for DataAccessGuard<'t, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

use crate::relations::{HasRelations, Relationship};

impl<'t, T> DataAccessGuard<'t, T>
where
    T: HasSchema,
{
    /// Include other related objects in a returned Dataset
    pub fn get<'g, R, Ship>(
        self,
        _relationship: impl Fn(<T as HasRelations>::Relation) -> Ship,
    ) -> Option<&'g [R]>
    where
        'g: 't,
        T: HasRelations,
        Ship: Relationship<R>,
        R: HasSchema,
        R: Send + Sync + HasSchema,
        <R as HasSchema>::Schema: TableInfo + TableColumns + UniqueIdentifier,
        <T as HasSchema>::Schema: TableInfo + TableColumns + UniqueIdentifier,
        <T as HasRelations>::Relation: Default,
    {
        // find the set of data that would fit
        for set in &self.sets.related {
            let into_t: Option<&[R]> = set.try_into().ok();
            if let Some(slice) = into_t {
                return Some(slice);
            }
        }
        None
    }
}
