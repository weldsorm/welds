use super::{AsFieldName, ClauseAdder, ClauseColVal};
use std::marker::PhantomData;
use welds_connections::Param;

/// Clauses for numeric types such as int, float, etc
pub struct Numeric<T> {
    col: String,
    field: String,
    _t: PhantomData<T>,
}

impl<T> AsFieldName<T> for Numeric<T> {
    fn colname(&self) -> &str {
        self.col.as_str()
    }
    fn fieldname(&self) -> &str {
        self.field.as_str()
    }
}

impl<T> Numeric<T>
where
    T: 'static + Clone + Send + Sync,
{
    pub fn new(col: impl Into<String>, field: impl Into<String>) -> Self {
        Self {
            col: col.into(),
            field: field.into(),
            _t: Default::default(),
        }
    }

    /// Will write SQL checking the value is equal to this (==)
    pub fn equal(self, v: impl Into<T>) -> Box<dyn ClauseAdder>
    where
        T: Param,
    {
        let cv = ClauseColVal::<T> {
            null_clause: false,
            not_clause: false,
            col: self.col,
            operator: "=",
            val: Some(v.into()),
        };
        Box::new(cv)
    }

    /// Will write SQL checking the value is equal to this (!=)
    pub fn not_equal(self, v: impl Into<T>) -> Box<dyn ClauseAdder>
    where
        T: Param,
    {
        let cv = ClauseColVal::<T> {
            null_clause: false,
            not_clause: true,
            col: self.col,
            operator: "!=",
            val: Some(v.into()),
        };
        Box::new(cv)
    }

    /// Will write SQL checking the value is greater than (>)
    pub fn gt(self, v: impl Into<T>) -> Box<dyn ClauseAdder>
    where
        T: Param,
    {
        let cv = ClauseColVal::<T> {
            null_clause: false,
            not_clause: false,
            col: self.col,
            operator: ">",
            val: Some(v.into()),
        };
        Box::new(cv)
    }

    /// Will write SQL checking the value is less than (<)
    pub fn lt(self, v: impl Into<T>) -> Box<dyn ClauseAdder>
    where
        T: Param,
    {
        let cv = ClauseColVal::<T> {
            null_clause: false,
            not_clause: false,
            col: self.col,
            operator: "<",
            val: Some(v.into()),
        };
        Box::new(cv)
    }

    /// Will write SQL checking the value is greater than or equal to (>=)
    pub fn gte(self, v: impl Into<T>) -> Box<dyn ClauseAdder>
    where
        T: Param,
    {
        let cv = ClauseColVal::<T> {
            null_clause: false,
            not_clause: false,
            col: self.col,
            operator: ">=",
            val: Some(v.into()),
        };
        Box::new(cv)
    }

    /// Will write SQL checking the value is less than or equal to (<=)
    pub fn lte(self, v: impl Into<T>) -> Box<dyn ClauseAdder>
    where
        T: Param,
    {
        let cv = ClauseColVal::<T> {
            null_clause: false,
            not_clause: false,
            col: self.col,
            operator: "<=",
            val: Some(v.into()),
        };
        Box::new(cv)
    }
}
