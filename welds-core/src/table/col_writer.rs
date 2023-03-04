use super::Column;

pub(crate) struct SelectWriter {
    write: fn(&Column) -> String,
}
impl SelectWriter {
    pub fn new<DB: DbSelectWriter>() -> Self {
        Self { write: DB::write }
    }
    pub fn write(&self, col: &Column) -> String {
        (self.write)(col)
    }
}

pub trait DbSelectWriter {
    fn write(col: &Column) -> String;
}

#[cfg(feature = "postgres")]
impl DbSelectWriter for sqlx::Postgres {
    fn write(col: &Column) -> String {
        format!("{}", col.name())
    }
}

#[cfg(feature = "sqlite")]
impl DbSelectWriter for sqlx::Sqlite {
    fn write(col: &Column) -> String {
        format!("{}", col.name())
    }
}

#[cfg(feature = "mysql")]
impl DbSelectWriter for sqlx::MySql {
    fn write(col: &Column) -> String {
        format!("{}", col.name())
    }
}

#[cfg(feature = "mssql")]
impl DbSelectWriter for sqlx::Mssql {
    fn write(col: &Column) -> String {
        let dbtype = mssql_type_overrides(col.dbtype());
        format!("cast({} as {}) as {}", col.name(), dbtype, col.name())
    }
}

fn mssql_type_overrides(dbtype: &str) -> &str {
    match dbtype {
        //
        "BitN" => "BIT",
        _ => dbtype,
    }
}
