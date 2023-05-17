use super::{AsFieldName, ClauseAdder, ClauseColVal};
use std::marker::PhantomData;

pub struct Basic<T> {
    col: String,
    field: String,
    _t: PhantomData<T>,
}

impl<T> AsFieldName<T> for Basic<T> {
    fn colname(&self) -> &str {
        self.col.as_str()
    }
    fn fieldname(&self) -> &str {
        self.field.as_str()
    }
}

impl<T> Basic<T>
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
}
