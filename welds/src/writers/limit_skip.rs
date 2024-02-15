use crate::Syntax;

pub struct LimitSkipWriter {
    syntax: Syntax,
}

impl LimitSkipWriter {
    pub fn new(syntax: Syntax) -> Self {
        Self { syntax }
    }
    pub fn skiplimit(&self, s: &Option<i64>, l: &Option<i64>) -> Option<String> {
        match self.syntax {
            #[cfg(feature = "mysql")]
            Syntax::Mysql => MySql::skiplimit(s, l),
            #[cfg(feature = "postgres")]
            Syntax::Postgres => Postgres::skiplimit(s, l),
            #[cfg(feature = "sqlite")]
            Syntax::Sqlite => Sqlite::skiplimit(s, l),
            #[cfg(feature = "mssql")]
            Syntax::Mssql => Mssql::skiplimit(s, l),
        }
    }
}

#[cfg(feature = "postgres")]
struct Postgres;
#[cfg(feature = "postgres")]
impl Postgres {
    fn skiplimit(s: &Option<i64>, l: &Option<i64>) -> Option<String> {
        if s.is_none() && l.is_none() {
            return None;
        }
        let s = s.unwrap_or(0);
        let l = l.unwrap_or(9999999);
        Some(format!("OFFSET {} LIMIT {}", s, l))
    }
}

#[cfg(feature = "sqlite")]
struct Sqlite;
#[cfg(feature = "sqlite")]
impl Sqlite {
    fn skiplimit(s: &Option<i64>, l: &Option<i64>) -> Option<String> {
        if s.is_none() && l.is_none() {
            return None;
        }
        let s = s.unwrap_or(0);
        let l = l.unwrap_or(9999999);
        Some(format!("LIMIT {limit} OFFSET {skip} ", limit = l, skip = s))
    }
}

#[cfg(feature = "mssql")]
struct Mssql;
#[cfg(feature = "mssql")]
impl Mssql {
    fn skiplimit(s: &Option<i64>, l: &Option<i64>) -> Option<String> {
        if s.is_none() && l.is_none() {
            return None;
        }
        let s = s.unwrap_or(0);
        let l = l.unwrap_or(9999999);
        Some(format!("OFFSET {} ROWS FETCH FIRST {} ROWS ONLY", s, l))
    }
}

#[cfg(feature = "mysql")]
struct MySql;
#[cfg(feature = "mysql")]
impl MySql {
    fn skiplimit(s: &Option<i64>, l: &Option<i64>) -> Option<String> {
        if s.is_none() && l.is_none() {
            return None;
        }
        let s = s.unwrap_or(0);
        let l = l.unwrap_or(9999999);
        Some(format!("LIMIT {}, {}", s, l))
    }
}
