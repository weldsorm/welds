use crate::table::Column;

pub(crate) struct ColumnWriter {
    excape: fn(&str) -> String,
    write: fn(&str, &Column) -> String,
}
impl ColumnWriter {
    pub fn new<DB: DbColumnWriter>() -> Self {
        Self {
            write: DB::write,
            excape: DB::excape,
        }
    }
    pub fn write(&self, prefix: &str, col: &Column) -> String {
        (self.write)(prefix, col)
    }
    pub fn excape(&self, name: &str) -> String {
        (self.excape)(name)
    }
}

pub trait DbColumnWriter {
    fn excape(name: &str) -> String;
    fn write(prefix: &str, col: &Column) -> String;
}

#[cfg(feature = "postgres")]
impl DbColumnWriter for sqlx::Postgres {
    fn excape(name: &str) -> String {
        format!("\"{}\"", name)
    }
    fn write(prefix: &str, col: &Column) -> String {
        let name = Self::excape(col.name());
        format!("{}.{}", prefix, name)
    }
}

#[cfg(feature = "sqlite")]
impl DbColumnWriter for sqlx::Sqlite {
    fn excape(name: &str) -> String {
        format!("\"{}\"", name)
    }
    fn write(prefix: &str, col: &Column) -> String {
        let name = Self::excape(col.name());
        format!("{}.{}", prefix, name)
    }
}

#[cfg(feature = "mysql")]
impl DbColumnWriter for sqlx::MySql {
    fn excape(name: &str) -> String {
        name.to_string()
    }
    fn write(prefix: &str, col: &Column) -> String {
        let name = Self::excape(col.name());
        format!("{}.{}", prefix, name)
    }
}

#[cfg(feature = "mssql")]
impl DbColumnWriter for sqlx::Mssql {
    fn excape(name: &str) -> String {
        format!("\"{}\"", name)
    }
    fn write(prefix: &str, col: &Column) -> String {
        let dbtype = mssql_type_overrides(col.dbtype());
        let name = Self::excape(col.name());
        format!("cast({}.{} as {}) as {}", prefix, name, dbtype, col.name())
    }
}

fn mssql_type_overrides(dbtype: &str) -> &str {
    match dbtype {
        //
        "BitN" => "BIT",
        _ => dbtype,
    }
}
