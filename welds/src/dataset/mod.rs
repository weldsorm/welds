use crate::model_traits::CheckRelationship;
use crate::model_traits::{HasSchema, TableColumns, TableInfo, UniqueIdentifier};
use crate::query::include::related_query::{RelatedSetAccesser, SetDowncast};
use crate::state::DbState;
use std::ops::Deref;

/// A Collection object that hold a set of data that has been
/// selected out of the database and its related objects
///
/// ```
/// use welds::prelude::*;
///
/// #[derive(Debug, Default, WeldsModel)]
/// #[welds(table = "customers")]
/// #[welds(HasMany(orders, Order, "customer_id"))]
/// struct Customer {
///     #[welds(primary_key)]
///     pub id: i32,
/// }
///
/// #[derive(Debug, Default, WeldsModel)]
/// #[welds(table = "orders")]
/// #[welds(BelongsTo(customer, Customer, "customer_id"))]
/// struct Order {
///     #[welds(primary_key)]
///     pub id: i32,
///     pub customer_id: i32,
/// }
///
/// async fn example(db: &dyn Client) -> welds::errors::Result<()> {
///     // make a minimal number of database calls to get both queries.
///     let dataset = Customer::all().include(|x| x.orders).run(db).await?;
///     for customer in dataset.iter() {
///         println!("CUSTOMER: {:?}", customer.id );
///         for order in customer.get(|c| c.orders).into_iter() {
///             println!("\tORDER: {:?}", order.id );
///         }
///     }
///     Ok(())
/// }
/// ```
///
pub struct DataSet<T> {
    // not sure if we want to use state or not
    primary: Vec<DbState<T>>,
    related: Vec<Box<dyn RelatedSetAccesser + Send>>,
}

impl<T> DataSet<T> {
    pub(crate) fn new(
        primary: Vec<DbState<T>>,
        related: Vec<Box<dyn RelatedSetAccesser + Send>>,
    ) -> Self {
        Self { primary, related }
    }

    pub fn iter(&self) -> DataSetIter<T> {
        DataSetIter {
            index: 0,
            inner: self,
        }
    }
}

pub struct DataSetIter<'t, T> {
    index: usize,
    inner: &'t DataSet<T>,
}

impl<'t, T> Iterator for DataSetIter<'t, T> {
    type Item = DataAccessGuard<'t, T>;
    fn next(&mut self) -> Option<Self::Item> {
        let obj: &DbState<T> = self.inner.primary.get(self.index)?;
        self.index += 1;
        Some(DataAccessGuard {
            inner: obj,
            sets: self.inner,
        })
    }
}

impl<T> DataSet<T> {
    /// Returns the count of the primary selected object
    pub fn len(&self) -> usize {
        self.primary.len()
    }

    /// Returns true if this dataset doesn't contain any data
    pub fn is_empty(&self) -> bool {
        self.primary.is_empty()
    }

    /// access a <T> at a given index.
    pub fn get(&self, index: usize) -> Option<DataAccessGuard<T>> {
        let obj = self.primary.get(index)?;
        Some(DataAccessGuard {
            inner: obj,
            sets: self,
        })
    }
}

pub struct DataAccessGuard<'t, T> {
    inner: &'t DbState<T>,
    sets: &'t DataSet<T>,
}

impl<'t, T> DataAccessGuard<'t, T> {
    pub fn as_ref(&self) -> &'t T {
        self.inner.as_ref()
    }
}

impl<T> Deref for DataAccessGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

use crate::relations::{HasRelations, RelationValue, Relationship};

impl<'t, T> DataAccessGuard<'t, T>
where
    T: HasSchema,
{
    /// Gets other objects related to this object.
    /// This is a subset of the included objects that are linked to self.
    /// Returns an empty list if the relationship was NOT included in the query.
    pub fn get<'g, R, Ship>(
        &self,
        relationship: impl Fn(<T as HasRelations>::Relation) -> Ship,
    ) -> Vec<&'g R>
    where
        'g: 't,
        't: 'g,
        T: HasRelations + RelationValue<R>,
        T: CheckRelationship,
        Ship: 'static + Relationship<R>,
        R: HasSchema + RelationValue<T>,
        R: 'static + Send + Sync + HasSchema,
        <R as HasSchema>::Schema: TableInfo + TableColumns + UniqueIdentifier,
        <T as HasSchema>::Schema: TableInfo + TableColumns + UniqueIdentifier,
        <T as HasRelations>::Relation: Default,
    {
        let t: &T = self.inner.as_ref();
        // find the set of data that would fit
        for rset in &self.sets.related {
            if let Some(related_set) = rset.downcast_ref::<R, Ship>() {
                // check that we are working with the same relationship
                let ship = relationship(Default::default());
                if related_set.ship == ship {
                    let mut set = Vec::default();
                    for d in &related_set.data {
                        if CheckRelationship::check(t, d) {
                            set.push(d);
                        }
                    }
                    return set;
                }
            }
        }
        Vec::default()
    }

    pub fn get_owned<'g, R, Ship>(
        &self,
        relationship: impl Fn(<T as HasRelations>::Relation) -> Ship,
    ) -> Vec<R>
    where
        'g: 't,
        't: 'g,
        T: HasRelations + RelationValue<R>,
        T: CheckRelationship,
        Ship: 'static + Relationship<R>,
        R: HasSchema + RelationValue<T> + ToOwned<Owned = R>,
        R: 'static + Send + Sync + HasSchema,
        <R as HasSchema>::Schema: TableInfo + TableColumns + UniqueIdentifier,
        <T as HasSchema>::Schema: TableInfo + TableColumns + UniqueIdentifier,
        <T as HasRelations>::Relation: Default,
    {
        let t: &T = self.inner.as_ref();
        // find the set of data that would fit
        for rset in &self.sets.related {
            if let Some(related_set) = rset.downcast_ref::<R, Ship>() {
                // check that we are working with the same relationship
                let ship = relationship(Default::default());
                if related_set.ship == ship {
                    let mut set = Vec::default();
                    for d in related_set.data.iter() {
                        if CheckRelationship::check(t, d) {
                            set.push(d.to_owned());
                        }
                    }
                    return set;
                }
            }
        }
        Vec::default()
    }
}
