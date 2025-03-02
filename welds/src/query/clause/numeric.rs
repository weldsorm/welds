use super::{AsFieldName, ClauseColVal, ClauseColValEqual, ClauseColValIn, ClauseColValList};
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
    pub fn equal(self, v: impl Into<T>) -> Box<ClauseColValEqual<T>>
    where
        T: Param,
    {
        let cv = ClauseColValEqual::<T> {
            null_clause: false,
            not_clause: false,
            col: self.col,
            operator: "=",
            val: Some(v.into()),
        };
        Box::new(cv)
    }

    /// Will write SQL checking the value is equal to this (!=)
    pub fn not_equal(self, v: impl Into<T>) -> Box<ClauseColVal<T>>
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
    pub fn gt(self, v: impl Into<T>) -> Box<ClauseColVal<T>>
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
    pub fn lt(self, v: impl Into<T>) -> Box<ClauseColVal<T>>
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
    pub fn gte(self, v: impl Into<T>) -> Box<ClauseColVal<T>>
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
    pub fn lte(self, v: impl Into<T>) -> Box<ClauseColVal<T>>
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

    /// Will write SQL checking for any matching value in from list
    /// NOTE: the negation of this operator is not_all(&[])
    #[cfg(feature = "postgres")]
    pub fn any<P>(self, slice: &[P]) -> Box<ClauseColValList<T>>
    where
        P: Into<T> + Clone,
        Vec<T>: Param,
    {
        let mut list: Vec<T> = Vec::default();
        for p in slice {
            list.push(p.clone().into());
        }
        let cv = ClauseColValList::<T> {
            col: self.col,
            operator: "= any",
            list,
        };
        Box::new(cv)
    }

    /// Will make sure the columns values does NOT match ALL values in the list
    #[cfg(feature = "postgres")]
    pub fn not_all<P>(self, slice: &[P]) -> Box<ClauseColValList<T>>
    where
        P: Into<T> + Clone,
        Vec<T>: Param,
    {
        let mut list: Vec<T> = Vec::default();
        for p in slice {
            list.push(p.clone().into());
        }
        let cv = ClauseColValList::<T> {
            col: self.col,
            operator: "!= all",
            list,
        };
        Box::new(cv)
    }

    /// Will write SQL "IN" to check that the value is in a list
    pub fn in_list<P>(self, slice: &[P]) -> Box<ClauseColValIn<T>>
    where
        P: Into<T> + Clone,
        T: Param,
    {
        let mut list = Vec::default();
        for param in slice {
            list.push(param.clone().into());
        }
        let c = ClauseColValIn::<T> {
            col: self.col,
            operator: "IN",
            list,
        };
        Box::new(c)
    }
}
