use super::{
    AsFieldName, AsOptField, ClauseColVal, ClauseColValEqual, ClauseColValIn, ClauseColValList,
};
use crate::query::optional::HasSomeNone;
use crate::query::optional::Optional;
use std::marker::PhantomData;
use welds_connections::Param;

pub struct NumericOpt<T> {
    col: String,
    field: String,
    _t: PhantomData<T>,
}

impl<T> AsFieldName<T> for NumericOpt<T> {
    fn colname(&self) -> &str {
        self.col.as_str()
    }
    fn fieldname(&self) -> &str {
        self.field.as_str()
    }
}

impl<T> AsOptField for NumericOpt<T> {}

impl<T> NumericOpt<T>
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

    pub fn gt(self, v: impl Into<Optional<T>>) -> Box<ClauseColVal<T>>
    where
        T: Param,
    {
        let opt = v.into();
        let is_none = opt.is_none();
        let val: Option<T> = opt.into();

        let cv = ClauseColVal::<T> {
            null_clause: is_none,
            not_clause: false,
            col: self.col,
            operator: ">",
            val,
        };
        Box::new(cv)
    }

    pub fn lt(self, v: impl Into<Optional<T>>) -> Box<ClauseColVal<T>>
    where
        T: Param,
    {
        let opt = v.into();
        //let is_none = opt.is_none();
        let val: Option<T> = opt.into();

        let cv = ClauseColVal::<T> {
            null_clause: val.is_none(),
            not_clause: false,
            col: self.col,
            operator: "<",
            val,
        };
        Box::new(cv)
    }

    pub fn gte(self, v: impl Into<Optional<T>>) -> Box<ClauseColVal<T>>
    where
        T: Param,
    {
        let opt = v.into();
        let is_none = opt.is_none();
        let val: Option<T> = opt.into();

        let cv = ClauseColVal::<T> {
            null_clause: is_none,
            not_clause: false,
            col: self.col,
            operator: ">=",
            val,
        };
        Box::new(cv)
    }

    pub fn lte(self, v: impl Into<Optional<T>>) -> Box<ClauseColVal<T>>
    where
        T: Param,
    {
        let opt = v.into();
        let is_none = opt.is_none();
        let val: Option<T> = opt.into();

        let cv = ClauseColVal::<T> {
            null_clause: is_none,
            not_clause: false,
            col: self.col,
            operator: "<=",
            val,
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
