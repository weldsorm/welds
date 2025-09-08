use super::Relationship;
use crate::model_traits::UniqueIdentifier;
use std::marker::PhantomData;

// use super::RelationshipCompare;
// use crate::model_traits::ForeignKeyPartialEq;
// use crate::model_traits::HasSchema;
// use crate::model_traits::PrimaryKeyValue;

pub struct ManualRelationship<T> {
    _t: PhantomData<T>,
    // These database column on self to use for this relationship
    self_key: &'static str,
    // The database column on the other table to use for this relationship
    other_key: &'static str,
}

impl<T> ManualRelationship<T> {
    pub fn using(self_col_key: &'static str, other_col_key: &'static str) -> ManualRelationship<T> {
        ManualRelationship {
            _t: Default::default(),
            self_key: self_col_key,
            other_key: other_col_key,
        }
    }
}

impl<T> PartialEq for ManualRelationship<T> {
    fn eq(&self, other: &Self) -> bool {
        self.self_key == other.self_key && self.other_key == other.other_key
    }
}
impl<T> Clone for ManualRelationship<T> {
    fn clone(&self) -> Self {
        Self {
            _t: Default::default(),
            self_key: self.self_key,
            other_key: self.other_key,
        }
    }
}

impl<R: Send> Relationship<R> for ManualRelationship<R> {
    fn my_key<ME, THEM>(&self) -> String
    where
        ME: UniqueIdentifier,
        THEM: UniqueIdentifier,
    {
        self.self_key.to_owned()
    }

    fn their_key<ME, THEM>(&self) -> String
    where
        ME: UniqueIdentifier,
        THEM: UniqueIdentifier,
    {
        self.other_key.to_owned()
    }
}

// RelationshipCompare is used for include Queries

//impl<T, R> RelationshipCompare<T, R> for Manual<R>
//where
//    Self: Relationship<R>,
//    T: PrimaryKeyValue + HasSchema,
//    T::Schema: UniqueIdentifier,
//    R: HasSchema,
//    R::Schema: UniqueIdentifier,
//    R: ForeignKeyPartialEq<T::PrimaryKeyType>,
//{
//    fn is_related(&self, source: &T, other: &R) -> bool {
//        let pk = source.primary_key_value();
//        let fk_field: String = Self::their_key::<R::Schema, T::Schema>(self);
//        other.eq(&fk_field, &pk)
//    }
//}
