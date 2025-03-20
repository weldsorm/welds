use super::Relationship;
use super::RelationshipCompare;
use crate::model_traits::ForeignKeyPartialEq;
use crate::model_traits::HasSchema;
use crate::model_traits::PrimaryKeyValue;
use crate::model_traits::UniqueIdentifier;
use std::marker::PhantomData;

pub struct BelongsToOne<T> {
    _t: PhantomData<T>,
    foreign_key: &'static str,
}

impl<T> PartialEq for BelongsToOne<T> {
    fn eq(&self, other: &Self) -> bool {
        self.foreign_key == other.foreign_key
    }
}

impl<T> Clone for BelongsToOne<T> {
    fn clone(&self) -> Self {
        BelongsToOne {
            _t: Default::default(),
            foreign_key: self.foreign_key,
        }
    }
}

impl<T> BelongsToOne<T> {
    pub fn using(fk: &'static str) -> BelongsToOne<T> {
        BelongsToOne {
            _t: Default::default(),
            foreign_key: fk,
        }
    }
}

impl<R: Send> Relationship<R> for BelongsToOne<R> {
    fn my_key<ME, THEM>(&self) -> String
    where
        ME: UniqueIdentifier,
        THEM: UniqueIdentifier,
    {
        THEM::id_column().name().to_owned()
    }
    fn their_key<ME, THEM>(&self) -> String
    where
        ME: UniqueIdentifier,
        THEM: UniqueIdentifier,
    {
        self.foreign_key.to_owned()
    }
}

impl<T, R> RelationshipCompare<T, R> for BelongsToOne<R>
where
    Self: Relationship<R>,
    T: PrimaryKeyValue + HasSchema,
    T::Schema: UniqueIdentifier,
    R: HasSchema,
    R::Schema: UniqueIdentifier,
    R: ForeignKeyPartialEq<T::PrimaryKeyType>,
{
    fn is_related(&self, source: &T, other: &R) -> bool {
        let pk = source.primary_key_value();
        let fk_field: String = Self::their_key::<R::Schema, T::Schema>(self);
        other.eq(&fk_field, &pk)
    }
}
