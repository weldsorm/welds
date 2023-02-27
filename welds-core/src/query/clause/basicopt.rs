use super::{ClauseColVal, QueryBuilderAdder};
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

    pub fn equal<'args, DB>(self, v: impl Into<T>) -> Box<dyn QueryBuilderAdder<'args, DB>>
    where
        DB: sqlx::Database,
        T: sqlx::Type<DB> + sqlx::Encode<'args, DB>,
    {
        let val = v.into();
        let cv = ClauseColVal::<T> {
            isnull_clause: val.is_none(),
            col: self.field,
            operator: "=",
            val,
        };
        Box::new(cv)
    }

    pub fn not_equal<'args, DB>(self, v: impl Into<T>) -> Box<dyn QueryBuilderAdder<'args, DB>>
    where
        DB: sqlx::Database,
        T: sqlx::Type<DB> + sqlx::Encode<'args, DB>,
    {
        let val = v.into();
        let cv = ClauseColVal::<T> {
            isnull_clause: val.is_none(),
            col: self.field,
            operator: "!=",
            val,
        };
        Box::new(cv)
    }
}
