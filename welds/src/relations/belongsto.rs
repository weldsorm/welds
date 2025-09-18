use super::Relationship;
use super::RelationshipCompare;
use crate::model_traits::ForeignKeyPartialEq;
use crate::model_traits::HasSchema;
use crate::model_traits::PrimaryKeyValue;
use crate::model_traits::UniqueIdentifier;
use std::marker::PhantomData;

pub struct BelongsTo<T, R> {
    _t: PhantomData<T>,
    _r: PhantomData<R>,
    foreign_key: &'static str,
}

impl<T, R> BelongsTo<T, R> {
    pub fn using(fk: &'static str) -> BelongsTo<T, R> {
        BelongsTo {
            _t: Default::default(),
            _r: Default::default(),
            foreign_key: fk,
        }
    }
}

// writing these by hand to ignore PhantomData
impl<T, R> PartialEq for BelongsTo<T, R> {
    fn eq(&self, other: &Self) -> bool {
        self.foreign_key == other.foreign_key
    }
}
impl<T, R> Clone for BelongsTo<T, R> {
    fn clone(&self) -> Self {
        Self {
            _t: Default::default(),
            _r: Default::default(),
            foreign_key: self.foreign_key,
        }
    }
}

impl<T: Send, R: Send> Relationship<T, R> for BelongsTo<T, R>
where
    R: HasSchema,
    <R as HasSchema>::Schema: UniqueIdentifier,
{
    fn my_key(&self) -> String {
        self.foreign_key.to_owned()
    }
    fn their_key(&self) -> String {
        <R as HasSchema>::Schema::id_column().name().to_owned()
    }
}

impl<T, R> RelationshipCompare<T, R> for BelongsTo<T, R>
where
    Self: Relationship<T, R>,
    R: PrimaryKeyValue + HasSchema,
    R::Schema: UniqueIdentifier,
    T: HasSchema,
    T::Schema: UniqueIdentifier,
    T: ForeignKeyPartialEq<R::PrimaryKeyType>,
{
    fn is_related(&self, source: &T, other: &R) -> bool {
        let pk = other.primary_key_value();
        let fk_field: String = Self::my_key(self);
        source.eq(&fk_field, &pk)
    }
}
