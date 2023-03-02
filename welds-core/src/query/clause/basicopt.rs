use super::{ClauseAdder, ClauseColVal};
use crate::query::optional::HasSomeNone;
use std::marker::PhantomData;

pub struct BasicOpt<T> {
    field: String,
    _t: PhantomData<T>,
}

impl<T> BasicOpt<T>
where
    T: 'static + HasSomeNone + Clone + Send,
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
        let val = v.into();

        let null_clause = if val.is_none() {
            Some(format!("{} IS NULL", self.field))
        } else {
            None
        };

        let cv = ClauseColVal::<T> {
            null_clause,
            col: self.field,
            operator: "=",
            val,
        };
        Box::new(cv)
    }

    pub fn not_equal<'args, DB>(self, v: impl Into<T>) -> Box<dyn ClauseAdder<'args, DB>>
    where
        DB: sqlx::Database,
        T: sqlx::Type<DB> + sqlx::Encode<'args, DB>,
    {
        let val = v.into();

        let null_clause = if val.is_none() {
            Some(format!("{} IS NOT NULL", self.field))
        } else {
            None
        };

        let cv = ClauseColVal::<T> {
            null_clause,
            col: self.field,
            operator: "!=",
            val,
        };
        Box::new(cv)
    }
}
