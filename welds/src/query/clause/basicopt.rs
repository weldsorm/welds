use super::{AsFieldName, AsOptField, ClauseColVal, ClauseColValEqual, ClauseColValIn};
use crate::query::optional::HasSomeNone;
use crate::query::optional::Optional;
use std::marker::PhantomData;
use welds_connections::Param;

#[derive(Clone)]
pub struct BasicOpt<T> {
    col: &'static str,
    field: &'static str,
    _t: PhantomData<T>,
}

impl<T> AsFieldName<T> for BasicOpt<T> {
    fn colname(&self) -> &'static str {
        self.col
    }
    fn fieldname(&self) -> &'static str {
        self.field
    }
}
impl<T: Clone> Copy for BasicOpt<T> {}

impl<T> AsOptField for BasicOpt<T> {}

impl<T> BasicOpt<T>
where
    T: 'static + Clone + Send + Sync,
{
    pub fn new(col: &'static str, field: &'static str) -> Self {
        Self {
            col: col.into(),
            field: field.into(),
            _t: Default::default(),
        }
    }

    pub fn equal(self, v: impl Into<Optional<T>>) -> Box<ClauseColValEqual<T>>
    where
        T: Param,
    {
        let opt = v.into();
        let is_none = opt.is_none();
        let val: Option<T> = opt.into();
        let cv = ClauseColValEqual::<T> {
            null_clause: is_none,
            not_clause: false,
            col: self.col,
            operator: "=",
            val,
        };
        Box::new(cv)
    }

    pub fn not_equal(self, v: impl Into<Optional<T>>) -> Box<ClauseColVal<T>>
    where
        T: Param,
    {
        let opt = v.into();
        let is_none = opt.is_none();
        let val: Option<T> = opt.into();
        let cv = ClauseColVal::<T> {
            null_clause: is_none,
            not_clause: true,
            col: self.col,
            operator: "!=",
            val,
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

    /// Will write SQL "NOT IN ()" to check that the value is not in a list
    pub fn not_in_list<P>(self, slice: &[P]) -> Box<ClauseColValIn<T>>
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
            operator: "NOT IN",
            list,
        };
        Box::new(c)
    }
}
