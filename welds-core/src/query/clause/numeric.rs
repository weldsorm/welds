use super::{ClauseColVal, QueryBuilderAdder};
use std::marker::PhantomData;

// Clauses for numeric types such as int, float, etc

pub struct Numeric<T> {
    field: String,
    _t: PhantomData<T>,
}

impl<'args, T> Numeric<T>
where
    T: Send + Clone + crate::row::ToRow<'args> + 'static,
{
    pub fn new(field: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            _t: Default::default(),
        }
    }

    pub fn equal(self, v: impl Into<T>) -> Box<dyn QueryBuilderAdder<'args>> {
        let cv = ClauseColVal::<T> {
            isnull_clause: false,
            col: self.field,
            operator: "=",
            val: v.into(),
        };
        Box::new(cv)
    }

    pub fn not_equal(self, v: impl Into<T>) -> Box<dyn QueryBuilderAdder<'args>> {
        let cv = ClauseColVal::<T> {
            isnull_clause: false,
            col: self.field,
            operator: "!=",
            val: v.into(),
        };
        Box::new(cv)
    }

    pub fn gt(self, v: impl Into<T>) -> Box<dyn QueryBuilderAdder<'args>> {
        let cv = ClauseColVal::<T> {
            isnull_clause: false,
            col: self.field,
            operator: ">",
            val: v.into(),
        };
        Box::new(cv)
    }
    pub fn lt(self, v: impl Into<T>) -> Box<dyn QueryBuilderAdder<'args>> {
        let cv = ClauseColVal::<T> {
            isnull_clause: false,
            col: self.field,
            operator: "<",
            val: v.into(),
        };
        Box::new(cv)
    }

    pub fn gte(self, v: impl Into<T>) -> Box<dyn QueryBuilderAdder<'args>> {
        let cv = ClauseColVal::<T> {
            isnull_clause: false,
            col: self.field,
            operator: ">=",
            val: v.into(),
        };
        Box::new(cv)
    }
    pub fn lte(self, v: impl Into<T>) -> Box<dyn QueryBuilderAdder<'args>> {
        let cv = ClauseColVal::<T> {
            isnull_clause: false,
            col: self.field,
            operator: "<=",
            val: v.into(),
        };
        Box::new(cv)
    }
}
