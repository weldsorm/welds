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
            Syntax::Mysql => MySql::skiplimit(s, l),
            Syntax::Postgres => Postgres::skiplimit(s, l),
            Syntax::Sqlite => Sqlite::skiplimit(s, l),
            Syntax::Mssql => Mssql::skiplimit(s, l),
        }
    }
}

struct Postgres;
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

struct Sqlite;
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

struct Mssql;
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

struct MySql;
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
