use crate::table::Column;

pub(crate) struct ColumnWriter {
    write: fn(&Column) -> String,
}
impl ColumnWriter {
    pub fn new<DB: DbColumnWriter>() -> Self {
        Self { write: DB::write }
    }
    pub fn write(&self, col: &Column) -> String {
        (self.write)(col)
    }
}

pub trait DbColumnWriter {
    fn write(col: &Column) -> String;
}

#[cfg(feature = "postgres")]
impl DbColumnWriter for sqlx::Postgres {
    fn write(col: &Column) -> String {
        format!("{}", col.name())
    }
}

#[cfg(feature = "sqlite")]
impl DbColumnWriter for sqlx::Sqlite {
    fn write(col: &Column) -> String {
        format!("{}", col.name())
    }
}

#[cfg(feature = "mysql")]
impl DbColumnWriter for sqlx::MySql {
    fn write(col: &Column) -> String {
        format!("{}", col.name())
    }
}

#[cfg(feature = "mssql")]
impl DbColumnWriter for sqlx::Mssql {
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
