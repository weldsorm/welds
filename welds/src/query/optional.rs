/// This is part of the fluent builder for queries.
/// We are using it instead of Option so we can have slightly different behavior
///
/// often you have a database that has null columns everywhere.
/// If all you want to do is query, it would get very annoying have to deal will Some/None
/// for everything that goes into the query. This allows for searching by
/// .where_col(|x| x.name.equal("bla") )
/// and now having to know if the DB column is null
///
/// This is TO sql only.

pub enum Optional<T> {
    Some(T),
    None,
}

pub trait HasSomeNone {
    fn is_none(&self) -> bool;
    fn is_some(&self) -> bool;
}

impl<T> HasSomeNone for Optional<T> {
    fn is_none(&self) -> bool {
        match self {
            Optional::Some(_) => false,
            Optional::None => true,
        }
    }
    fn is_some(&self) -> bool {
        match self {
            Optional::Some(_) => true,
            Optional::None => false,
        }
    }
}

impl<T> From<T> for Optional<T> {
    fn from(inner: T) -> Optional<T> {
        Optional::Some(inner)
    }
}

impl<T> From<Optional<T>> for Option<T> {
    fn from(opt: Optional<T>) -> Option<T> {
        match opt {
            Optional::Some(x) => Some(x),
            Optional::None => None,
        }
    }
}
impl<T> From<Option<T>> for Optional<T> {
    fn from(opt: Option<T>) -> Optional<T> {
        match opt {
            Option::Some(x) => Optional::Some(x),
            Option::None => Optional::None,
        }
    }
}

impl From<&str> for Optional<String> {
    fn from(inner: &str) -> Optional<String> {
        Optional::Some(inner.into())
    }
}
impl From<&String> for Optional<String> {
    fn from(inner: &String) -> Optional<String> {
        Optional::Some(inner.into())
    }
}

impl From<&Option<&str>> for Optional<String> {
    fn from(inner: &Option<&str>) -> Optional<String> {
        match inner {
            Option::Some(x) => Optional::Some(x.to_string()),
            Option::None => Optional::None,
        }
    }
}
impl From<&Option<String>> for Optional<String> {
    fn from(inner: &Option<String>) -> Optional<String> {
        match inner {
            Option::Some(x) => Optional::Some(x.clone()),
            Option::None => Optional::None,
        }
    }
}

impl<T> Clone for Optional<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Optional::Some(x) => Optional::Some(x.clone()),
            Optional::None => Optional::None,
        }
    }
}

//use welds_connections::Param;

//impl<T> Param for Optional<T> where T: Param {}

//use sqlx::{Database, Encode, Type};
//
//impl<T, DB> Type<DB> for Optional<T>
//where
//    DB: Database,
//    T: Type<DB>,
//    Option<T>: Type<DB>,
//{
//    fn type_info() -> <DB as Database>::TypeInfo {
//        Option::<T>::type_info()
//    }
//    fn compatible(ty: &<DB as Database>::TypeInfo) -> bool {
//        Option::<T>::compatible(ty)
//    }
//}
//
//impl<'q, T, DB> Encode<'q, DB> for Optional<T>
//where
//    DB: Database,
//    T: Encode<'q, DB>,
//    Option<T>: Encode<'q, DB>,
//{
//    fn encode(
//        self,
//        buf: &mut <DB as sqlx::database::HasArguments<'q>>::ArgumentBuffer,
//    ) -> sqlx::encode::IsNull
//    where
//        Self: Sized,
//    {
//        let opt: Option<T> = self.into();
//        match opt {
//            Some(x) => T::encode(x, buf),
//            None => Option::<T>::encode(opt, buf),
//        }
//    }
//
//    fn encode_by_ref(
//        &self,
//        buf: &mut <DB as sqlx::database::HasArguments<'q>>::ArgumentBuffer,
//    ) -> sqlx::encode::IsNull {
//        match self {
//            Optional::Some(x) => T::encode_by_ref(x, buf),
//            Optional::None => Option::<T>::encode_by_ref(&None, buf),
//        }
//    }
//    fn produces(&self) -> Option<<DB as Database>::TypeInfo> {
//        match self {
//            Optional::Some(x) => T::produces(x),
//            Optional::None => Option::<T>::produces(&None),
//        }
//    }
//
//    fn size_hint(&self) -> usize {
//        match self {
//            Optional::Some(x) => T::size_hint(x),
//            Optional::None => Option::<T>::size_hint(&None),
//        }
//    }
//}
