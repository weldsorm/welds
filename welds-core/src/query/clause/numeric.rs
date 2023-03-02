use super::{ClauseAdder, ClauseColVal};
use std::marker::PhantomData;

// Clauses for numeric types such as int, float, etc

pub struct Numeric<T> {
    field: String,
    _t: PhantomData<T>,
}

impl<T> Numeric<T>
where
    T: 'static + Clone + Send, //T: 'static + Clone + Send + sqlx::Type<DB> + sqlx::Encode<'args, DB>,
{
    pub fn new(field: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            _t: Default::default(),
        }
    }

    pub fn equal<'args, DB>(self, v: impl Into<T>) -> Box<dyn ClauseAdder<'args, DB>>
    where
        DB: sqlx::Database,
        T: sqlx::Type<DB> + sqlx::Encode<'args, DB>,
    {
        let cv = ClauseColVal::<T> {
            null_clause: None,
            col: self.field,
            operator: "=",
            val: v.into(),
        };
        Box::new(cv)
    }

    pub fn not_equal<'args, DB>(self, v: impl Into<T>) -> Box<dyn ClauseAdder<'args, DB>>
    where
        DB: sqlx::Database,
        T: sqlx::Type<DB> + sqlx::Encode<'args, DB>,
    {
        let cv = ClauseColVal::<T> {
            null_clause: None,
            col: self.field,
            operator: "!=",
            val: v.into(),
        };
        Box::new(cv)
    }

    pub fn gt<'args, DB>(self, v: impl Into<T>) -> Box<dyn ClauseAdder<'args, DB>>
    where
        DB: sqlx::Database,
        T: sqlx::Type<DB> + sqlx::Encode<'args, DB>,
    {
        let cv = ClauseColVal::<T> {
            null_clause: None,
            col: self.field,
            operator: ">",
            val: v.into(),
        };
        Box::new(cv)
    }
    pub fn lt<'args, DB>(self, v: impl Into<T>) -> Box<dyn ClauseAdder<'args, DB>>
    where
        DB: sqlx::Database,
        T: sqlx::Type<DB> + sqlx::Encode<'args, DB>,
    {
        let cv = ClauseColVal::<T> {
            null_clause: None,
            col: self.field,
            operator: "<",
            val: v.into(),
        };
        Box::new(cv)
    }

    pub fn gte<'args, DB>(self, v: impl Into<T>) -> Box<dyn ClauseAdder<'args, DB>>
    where
        DB: sqlx::Database,
        T: sqlx::Type<DB> + sqlx::Encode<'args, DB>,
    {
        let cv = ClauseColVal::<T> {
            null_clause: None,
            col: self.field,
            operator: ">=",
            val: v.into(),
        };
        Box::new(cv)
    }

    pub fn lte<'args, DB>(self, v: impl Into<T>) -> Box<dyn ClauseAdder<'args, DB>>
    where
        DB: sqlx::Database,
        T: sqlx::Type<DB> + sqlx::Encode<'args, DB>,
    {
        let cv = ClauseColVal::<T> {
            null_clause: None,
            col: self.field,
            operator: "<=",
            val: v.into(),
        };
        Box::new(cv)
    }
}
