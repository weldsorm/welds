use super::{AsFieldName, ClauseAdder, ClauseColVal};
use std::marker::PhantomData;

pub struct Text<T> {
    col: String,
    field: String,
    _t: PhantomData<T>,
}

impl<T> AsFieldName<T> for Text<T> {
    fn colname(&self) -> &str {
        self.col.as_str()
    }
    fn fieldname(&self) -> &str {
        self.field.as_str()
    }
}

impl<T> Text<T>
where
    T: 'static + Clone + Send,
{
    pub fn new(col: impl Into<String>, field: impl Into<String>) -> Self {
        Self {
            col: col.into(),
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
            null_clause: false,
            not_clause: false,
            col: self.col,
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
            null_clause: false,
            not_clause: true,
            col: self.col,
            operator: "!=",
            val: v.into(),
        };
        Box::new(cv)
    }

    pub fn like<'args, DB>(self, v: impl Into<T>) -> Box<dyn ClauseAdder<'args, DB>>
    where
        DB: sqlx::Database,
        T: sqlx::Type<DB> + sqlx::Encode<'args, DB>,
    {
        let cv = ClauseColVal::<T> {
            null_clause: false,
            not_clause: false,
            col: self.col,
            operator: "like",
            val: v.into(),
        };
        Box::new(cv)
    }
    pub fn not_like<'args, DB>(self, v: impl Into<T>) -> Box<dyn ClauseAdder<'args, DB>>
    where
        DB: sqlx::Database,
        T: sqlx::Type<DB> + sqlx::Encode<'args, DB>,
    {
        let cv = ClauseColVal::<T> {
            null_clause: false,
            not_clause: true,
            col: self.col,
            operator: "not like",
            val: v.into(),
        };
        Box::new(cv)
    }
    pub fn ilike<'args, DB>(self, v: impl Into<T>) -> Box<dyn ClauseAdder<'args, DB>>
    where
        DB: sqlx::Database,
        T: sqlx::Type<DB> + sqlx::Encode<'args, DB>,
    {
        let cv = ClauseColVal::<T> {
            null_clause: false,
            not_clause: false,
            col: self.col,
            operator: "ilike",
            val: v.into(),
        };
        Box::new(cv)
    }
    pub fn not_ilike<'args, DB>(self, v: impl Into<T>) -> Box<dyn ClauseAdder<'args, DB>>
    where
        DB: sqlx::Database,
        T: sqlx::Type<DB> + sqlx::Encode<'args, DB>,
    {
        let cv = ClauseColVal::<T> {
            null_clause: false,
            not_clause: true,
            col: self.col,
            operator: "not ilike",
            val: v.into(),
        };
        Box::new(cv)
    }
}
