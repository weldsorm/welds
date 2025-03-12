use crate::model_traits::UniqueIdentifier;
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

// writing these by hand to ignore PhantomData
impl<T> PartialEq for BelongsTo<T> {
    fn eq(&self, other: &Self) -> bool {
        self.foreign_key == other.foreign_key
    }
}
impl<T> Clone for BelongsTo<T> {
    fn clone(&self) -> Self {
        Self {
            _t: Default::default(),
            foreign_key: self.foreign_key,
        }
    }
}

impl<R: Send> Relationship<R> for BelongsTo<R> {
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

impl<T> PartialEq for HasMany<T> {
    fn eq(&self, other: &Self) -> bool {
        self.foreign_key == other.foreign_key
    }
}
impl<T> Clone for HasMany<T> {
    fn clone(&self) -> Self {
        Self {
            _t: Default::default(),
            foreign_key: self.foreign_key,
        }
    }
}

impl<R: Send> Relationship<R> for HasMany<R> {
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

impl<T> PartialEq for HasOne<T> {
    fn eq(&self, other: &Self) -> bool {
        self.foreign_key == other.foreign_key
    }
}
impl<T> Clone for HasOne<T> {
    fn clone(&self) -> Self {
        Self {
            _t: Default::default(),
            foreign_key: self.foreign_key,
        }
    }
}

impl<T> HasOne<T> {
    pub fn using(fk: &'static str) -> HasOne<T> {
        HasOne {
            _t: Default::default(),
            foreign_key: fk,
        }
    }
}

impl<R: Send> Relationship<R> for HasOne<R> {
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

pub trait Relationship<R>: Clone + PartialEq + Send {
    fn their_key<R2, T>(&self) -> String
    where
        T: UniqueIdentifier,
        R2: UniqueIdentifier;

    fn my_key<R2, T>(&self) -> String
    where
        T: UniqueIdentifier,
        R2: UniqueIdentifier;
}

pub trait HasRelations {
    type Relation: Default;
}

pub trait RelationAdder {}
