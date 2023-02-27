use super::{ClauseColVal, QueryBuilderAdder};
use std::marker::PhantomData;

pub struct Basic<T> {
    field: String,
    _t: PhantomData<T>,
    //_db: PhantomData<DB>,
}

impl<T> Basic<T>
where
    //    DB: sqlx::Database,
    T: 'static + Clone + Send,
{
    pub fn new(field: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            _t: Default::default(),
            //_db: Default::default(),
        }
    }

    pub fn equal<'args, DB>(self, v: impl Into<T>) -> Box<dyn QueryBuilderAdder<'args, DB>>
    where
        DB: sqlx::Database,
        T: sqlx::Type<DB> + sqlx::Encode<'args, DB>,
    {
        let cv = ClauseColVal::<T> {
            isnull_clause: false,
            col: self.field,
            operator: "=",
            val: v.into(),
        };
        Box::new(cv)
    }

    pub fn not_equal<'args, DB>(self, v: impl Into<T>) -> Box<dyn QueryBuilderAdder<'args, DB>>
    where
        DB: sqlx::Database,
        T: sqlx::Type<DB> + sqlx::Encode<'args, DB>,
    {
        let cv = ClauseColVal::<T> {
            isnull_clause: false,
            col: self.field,
            operator: "!=",
            val: v.into(),
        };
        Box::new(cv)
    }
}
