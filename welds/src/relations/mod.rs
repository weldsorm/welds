use crate::table::UniqueIdentifier;
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
    fn my_key<DB, ME, THEM>(&self) -> String
    where
        DB: sqlx::Database,
        ME: UniqueIdentifier<DB>,
        THEM: UniqueIdentifier<DB>,
    {
        self.foreign_key.to_owned()
    }
    fn their_key<DB, ME, THEM>(&self) -> String
    where
        DB: sqlx::Database,
        ME: UniqueIdentifier<DB>,
        THEM: UniqueIdentifier<DB>,
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
    fn my_key<DB, ME, THEM>(&self) -> String
    where
        DB: sqlx::Database,
        ME: UniqueIdentifier<DB>,
        THEM: UniqueIdentifier<DB>,
    {
        THEM::id_column().name().to_owned()
    }
    fn their_key<DB, ME, THEM>(&self) -> String
    where
        DB: sqlx::Database,
        ME: UniqueIdentifier<DB>,
        THEM: UniqueIdentifier<DB>,
    {
        self.foreign_key.to_owned()
    }
}

pub trait Relationship<R> {
    fn their_key<DB, R2, T>(&self) -> String
    where
        T: UniqueIdentifier<DB>,
        R2: UniqueIdentifier<DB>,
        DB: sqlx::Database;

    fn my_key<DB, R2, T>(&self) -> String
    where
        T: UniqueIdentifier<DB>,
        R2: UniqueIdentifier<DB>,
        DB: sqlx::Database;
}

pub trait HasRelations {
    type Relation: Default;
}

pub trait RelationAdder {}
