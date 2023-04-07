pub(crate) struct LimitSkipWriter {
    skiplimit: fn(&Option<i64>, &Option<i64>) -> Option<String>,
}

impl LimitSkipWriter {
    pub fn new<DB: DbLimitSkipWriter>() -> Self {
        Self {
            skiplimit: DB::skiplimit,
        }
    }
    pub fn skiplimit(&self, s: &Option<i64>, l: &Option<i64>) -> Option<String> {
        (self.skiplimit)(s, l)
    }
}

pub trait DbLimitSkipWriter {
    fn skiplimit(s: &Option<i64>, l: &Option<i64>) -> Option<String>;
}

#[cfg(feature = "postgres")]
impl DbLimitSkipWriter for sqlx::Postgres {
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
impl DbLimitSkipWriter for sqlx::Sqlite {
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
impl DbLimitSkipWriter for sqlx::Mssql {
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
impl DbLimitSkipWriter for sqlx::MySql {
    fn skiplimit(s: &Option<i64>, l: &Option<i64>) -> Option<String> {
        if s.is_none() && l.is_none() {
            return None;
        }
        let s = s.unwrap_or(0);
        let l = l.unwrap_or(9999999);
        Some(format!("LIMIT {}, {}", s, l))
    }
}
