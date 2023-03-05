pub(crate) struct LimitSkipWriter {
    limit: fn(i64) -> String,
    skip: fn(i64) -> String,
}

impl LimitSkipWriter {
    pub fn new<DB: DbLimitSkipWriter>() -> Self {
        Self {
            limit: DB::limit,
            skip: DB::skip,
        }
    }
    pub fn limit(&self, x: Option<i64>) -> String {
        match x {
            Some(x) => (self.limit)(x),
            None => "".to_owned(),
        }
    }
    pub fn skip(&self, x: Option<i64>) -> String {
        match x {
            Some(x) => (self.skip)(x),
            None => "".to_owned(),
        }
    }
}

pub trait DbLimitSkipWriter {
    fn limit(x: i64) -> String;
    fn skip(x: i64) -> String;
}

#[cfg(feature = "postgres")]
impl DbLimitSkipWriter for sqlx::Postgres {
    fn limit(x: i64) -> String {
        format!("LIMIT {}", x)
    }
    fn skip(x: i64) -> String {
        format!("OFFSET {}", x)
    }
}

#[cfg(feature = "sqlite")]
impl DbLimitSkipWriter for sqlx::Sqlite {
    fn limit(x: i64) -> String {
        format!("LIMIT {}", x)
    }
    fn skip(x: i64) -> String {
        format!("OFFSET {}", x)
    }
}

#[cfg(feature = "mssql")]
impl DbLimitSkipWriter for sqlx::Mssql {
    fn limit(x: i64) -> String {
        format!("FETCH NEXT {} ROWS ONLY", x)
    }
    fn skip(x: i64) -> String {
        format!("OFFSET {} ROWS", x)
    }
}

#[cfg(feature = "mysql")]
impl DbLimitSkipWriter for sqlx::MySql {
    fn limit(x: i64) -> String {
        format!("LIMIT {}", x)
    }
    fn skip(x: i64) -> String {
        format!("OFFSET {}", x)
    }
}
