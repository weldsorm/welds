use crate::model_traits::{HasSchema, UniqueIdentifier};
use std::hash::Hash;
use std::marker::PhantomData;

pub struct BelongsTo<T> {
    _t: PhantomData<T>,
    foreign_key: &'static str,
}

impl<T> BelongsTo<T> {
    pub fn using(fk: &'static str) -> BelongsTo<T> {
        BelongsTo {
            _t: Default::default(),
            foreign_key: fk,
        }
    }
}

impl<R> Relationship<R> for BelongsTo<R> {
    fn my_key<ME, THEM>(&self) -> String
    where
        ME: UniqueIdentifier,
        THEM: UniqueIdentifier,
    {
        self.foreign_key.to_owned()
    }
    fn their_key<ME, THEM>(&self) -> String
    where
        ME: UniqueIdentifier,
        THEM: UniqueIdentifier,
    {
        ME::id_column().name().to_owned()
    }
}

pub struct HasMany<T> {
    _t: PhantomData<T>,
    foreign_key: &'static str,
}

impl<T> HasMany<T> {
    pub fn using(fk: &'static str) -> HasMany<T> {
        HasMany {
            _t: Default::default(),
            foreign_key: fk,
        }
    }
}

impl<R> Relationship<R> for HasMany<R> {
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

pub struct HasOne<T> {
    _t: PhantomData<T>,
    foreign_key: &'static str,
}

impl<T> HasOne<T> {
    pub fn using(fk: &'static str) -> HasOne<T> {
        HasOne {
            _t: Default::default(),
            foreign_key: fk,
        }
    }
}

impl<R> Relationship<R> for HasOne<R> {
    fn my_key<ME, THEM>(&self) -> String
    where
        ME: UniqueIdentifier,
        THEM: UniqueIdentifier,
    {
        self.foreign_key.to_owned()
    }
    fn their_key<ME, THEM>(&self) -> String
    where
        ME: UniqueIdentifier,
        THEM: UniqueIdentifier,
    {
        ME::id_column().name().to_owned()
    }
}

pub struct BelongsToOne<T> {
    _t: PhantomData<T>,
    foreign_key: &'static str,
}

impl<T> BelongsToOne<T> {
    pub fn using(fk: &'static str) -> BelongsToOne<T> {
        BelongsToOne {
            _t: Default::default(),
            foreign_key: fk,
        }
    }
}

impl<R> Relationship<R> for BelongsToOne<R> {
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

pub trait Relationship<R> {
    fn their_key<R2, T>(&self) -> String
    where
        T: UniqueIdentifier,
        R2: UniqueIdentifier;

    fn my_key<R2, T>(&self) -> String
    where
        T: UniqueIdentifier,
        R2: UniqueIdentifier;
}

pub trait Related<R> {}

pub trait BelongsToFkValue<R>: Sized
where
    Self: HasSchema + Related<BelongsTo<R>>,
    R: HasSchema + Related<HasMany<Self>>
{
    type FkVal: Hash + Eq + 'static;

    /// Returns the value of a model's foreign key for relationship with <T>
    fn fk_value<T>(&self) -> Self::FkVal;
}

pub trait HasRelations {
    type Relation: Default;
}

pub trait RelationAdder {}
