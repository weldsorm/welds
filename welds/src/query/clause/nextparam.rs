pub trait DbParam {
    fn next(i: usize) -> String;
    fn max_params() -> u32;
}

#[cfg(feature = "postgres")]
impl DbParam for sqlx::Postgres {
    fn next(i: usize) -> String {
        format!("${}", i)
    }
    fn max_params() -> u32 {
        65535
    }
}

#[cfg(feature = "sqlite")]
impl DbParam for sqlx::Sqlite {
    fn next(_i: usize) -> String {
        "?".to_string()
    }
    fn max_params() -> u32 {
        999
    }
}

#[cfg(feature = "mssql")]
impl DbParam for sqlx::Mssql {
    fn next(i: usize) -> String {
        format!("@p{}", i)
    }
    fn max_params() -> u32 {
        60
        //2100
    }
}
#[cfg(feature = "mysql")]
impl DbParam for sqlx::MySql {
    fn next(_i: usize) -> String {
        "?".to_string()
    }
    fn max_params() -> u32 {
        64000
    }
}

use std::sync::{Arc, Mutex};

pub struct NextParam {
    i: Arc<Mutex<usize>>,
    db_next: fn(usize) -> String,
}
impl NextParam {
    pub fn new<T: DbParam>() -> Self {
        Self {
            i: Arc::new(Mutex::new(1)),
            db_next: T::next,
        }
    }
    pub fn next(&self) -> String {
        let lock = self.i.clone();
        let mut i = lock.lock().unwrap();
        let p = (self.db_next)(*i);
        *i += 1;
        p
    }
}

#[test]
fn pg_should_use_dollar_signs_with_numbers() {
    let p = NextParam::new::<sqlx::Postgres>();
    assert_eq!(p.next(), "$1");
    assert_eq!(p.next(), "$2");
    assert_eq!(p.next(), "$3");
    assert_eq!(p.next(), "$4");
}

#[test]
fn mssql_should_use_at_signs_with_numbers() {
    let p = NextParam::new::<sqlx::Mssql>();
    assert_eq!(p.next(), "@p1");
    assert_eq!(p.next(), "@p2");
    assert_eq!(p.next(), "@p3");
    assert_eq!(p.next(), "@p4");
}

#[test]
fn mysql_should_use_question_marks() {
    let p = NextParam::new::<sqlx::MySql>();
    assert_eq!(p.next(), "?");
    assert_eq!(p.next(), "?");
    assert_eq!(p.next(), "?");
    assert_eq!(p.next(), "?");
}

#[test]
fn sqlite_should_use_question_marks() {
    let p = NextParam::new::<sqlx::Sqlite>();
    assert_eq!(p.next(), "?");
    assert_eq!(p.next(), "?");
    assert_eq!(p.next(), "?");
    assert_eq!(p.next(), "?");
}
