pub(crate) struct CountWriter {
    count: fn(Option<&str>) -> String,
}

impl CountWriter {
    pub fn new<DB: DbCountWriter>() -> Self {
        Self { count: DB::count }
    }
    pub fn count(&self, x: Option<&str>) -> String {
        (self.count)(x)
    }
}

pub trait DbCountWriter {
    fn count(x: Option<&str>) -> String;
}

#[cfg(feature = "postgres")]
impl DbCountWriter for sqlx::Postgres {
    fn count(x: Option<&str>) -> String {
        let x = x.unwrap_or("*");
        format!("CAST( COUNT({}) as BIGINT )", x)
    }
}

#[cfg(feature = "sqlite")]
impl DbCountWriter for sqlx::Sqlite {
    fn count(x: Option<&str>) -> String {
        let x = x.unwrap_or("*");
        format!("CAST( COUNT({}) as BIGINT )", x)
    }
}

#[cfg(feature = "mssql")]
impl DbCountWriter for sqlx::Mssql {
    fn count(x: Option<&str>) -> String {
        let x = x.unwrap_or("*");
        format!("CAST( COUNT({}) as BIGINT )", x)
    }
}

#[cfg(feature = "mysql")]
impl DbCountWriter for sqlx::MySql {
    fn count(x: Option<&str>) -> String {
        let x = x.unwrap_or("*");
        format!("COUNT({})", x)
    }
}
