use super::{AsFieldName, ClauseAdder, ClauseColVal};
use crate::query::optional::HasSomeNone;
use std::marker::PhantomData;

pub struct TextOpt<T> {
    field: String,
    _t: PhantomData<T>,
}

impl<T> AsFieldName<T> for TextOpt<T> {
    fn fieldname(&self) -> &str {
        self.field.as_str()
    }
}

impl<T> TextOpt<T>
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

        let cv = ClauseColVal::<T> {
            null_clause: val.is_none(),
            not_clause: false,
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

        let cv = ClauseColVal::<T> {
            null_clause: val.is_none(),
            not_clause: true,
            col: self.field,
            operator: "!=",
            val,
        };
        Box::new(cv)
    }

    pub fn like<'args, DB>(self, v: impl Into<T>) -> Box<dyn ClauseAdder<'args, DB>>
    where
        DB: sqlx::Database,
        T: sqlx::Type<DB> + sqlx::Encode<'args, DB>,
    {
        let val = v.into();

        let cv = ClauseColVal::<T> {
            null_clause: val.is_none(),
            not_clause: false,
            col: self.field,
            operator: "like",
            val,
        };
        Box::new(cv)
    }

    pub fn not_like<'args, DB>(self, v: impl Into<T>) -> Box<dyn ClauseAdder<'args, DB>>
    where
        DB: sqlx::Database,
        T: sqlx::Type<DB> + sqlx::Encode<'args, DB>,
    {
        let val = v.into();
        let cv = ClauseColVal::<T> {
            null_clause: val.is_none(),
            not_clause: true,
            col: self.field,
            operator: "not like",
            val,
        };
        Box::new(cv)
    }
    pub fn ilike<'args, DB>(self, v: impl Into<T>) -> Box<dyn ClauseAdder<'args, DB>>
    where
        DB: sqlx::Database,
        T: sqlx::Type<DB> + sqlx::Encode<'args, DB>,
    {
        let val = v.into();
        let cv = ClauseColVal::<T> {
            null_clause: val.is_none(),
            not_clause: false,
            col: self.field,
            operator: "ilike",
            val,
        };
        Box::new(cv)
    }
    pub fn not_ilike<'args, DB>(self, v: impl Into<T>) -> Box<dyn ClauseAdder<'args, DB>>
    where
        DB: sqlx::Database,
        T: sqlx::Type<DB> + sqlx::Encode<'args, DB>,
    {
        let val = v.into();
        let cv = ClauseColVal::<T> {
            null_clause: val.is_none(),
            not_clause: true,
            col: self.field,
            operator: "not ilike",
            val,
        };
        Box::new(cv)
    }
}
