use super::Relationship;
use super::RelationshipCompare;
use crate::model_traits::ForeignKeyPartialEq;
use crate::model_traits::HasSchema;
use crate::model_traits::PrimaryKeyValue;
use crate::model_traits::UniqueIdentifier;
use std::marker::PhantomData;

pub struct HasOne<T, R> {
    _t: PhantomData<T>,
    _r: PhantomData<R>,
    foreign_key: &'static str,
}

impl<T, R> PartialEq for HasOne<T, R> {
    fn eq(&self, other: &Self) -> bool {
        self.foreign_key == other.foreign_key
    }
}
impl<T, R> Clone for HasOne<T, R> {
    fn clone(&self) -> Self {
        Self {
            _t: Default::default(),
            _r: Default::default(),
            foreign_key: self.foreign_key,
        }
    }
}

impl<T, R> HasOne<T, R> {
    pub fn using(fk: &'static str) -> HasOne<T, R> {
        HasOne {
            _t: Default::default(),
            _r: Default::default(),
            foreign_key: fk,
        }
    }
}

impl<T: Send, R: Send> Relationship<T, R> for HasOne<T, R>
where
    T: HasSchema,
    <T as HasSchema>::Schema: UniqueIdentifier,
{
    fn my_key(&self) -> String {
        <T as HasSchema>::Schema::id_column().name().to_owned()
    }
    fn their_key(&self) -> String {
        self.foreign_key.to_owned()
    }
}

impl<T, R> RelationshipCompare<T, R> for HasOne<T, R>
where
    Self: Relationship<T, R>,
    T: PrimaryKeyValue + HasSchema,
    T::Schema: UniqueIdentifier,
    R: HasSchema,
    R::Schema: UniqueIdentifier,
    R: ForeignKeyPartialEq<T::PrimaryKeyType>,
{
    fn is_related(&self, source: &T, other: &R) -> bool {
        let pk = source.primary_key_value();
        let fk_field: String = Self::their_key(self);
        other.eq(&fk_field, &pk)
    }
}
