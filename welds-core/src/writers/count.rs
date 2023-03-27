pub(crate) struct CountWriter {
    count: fn(Option<&str>, Option<&str>) -> String,
}

impl CountWriter {
    pub fn new<DB: DbCountWriter>() -> Self {
        Self { count: DB::count }
    }
    pub fn count(&self, prefix: Option<&str>, x: Option<&str>) -> String {
        (self.count)(prefix, x)
    }
}

pub trait DbCountWriter {
    fn count(prefix: Option<&str>, x: Option<&str>) -> String;
}

#[cfg(feature = "postgres")]
impl DbCountWriter for sqlx::Postgres {
    fn count(prefix: Option<&str>, x: Option<&str>) -> String {
        let mut x = x.unwrap_or("*").to_owned();
        if let Some(prefix) = prefix {
            x = format!("{}.{}", prefix, x);
        }
        format!("CAST( COUNT({}) as BIGINT )", x)
    }
}

#[cfg(feature = "sqlite")]
impl DbCountWriter for sqlx::Sqlite {
    fn count(_prefix: Option<&str>, x: Option<&str>) -> String {
        let x = x.unwrap_or("*");
        format!("CAST( COUNT({}) as BIGINT )", x)
    }
}

#[cfg(feature = "mssql")]
impl DbCountWriter for sqlx::Mssql {
    fn count(_prefix: Option<&str>, x: Option<&str>) -> String {
        let x = x.unwrap_or("*");
        format!("CAST( COUNT({}) as BIGINT )", x)
    }
}

#[cfg(feature = "mysql")]
impl DbCountWriter for sqlx::MySql {
    fn count(_prefix: Option<&str>, x: Option<&str>) -> String {
        let x = x.unwrap_or("*");
        format!("COUNT({})", x)
    }
}
