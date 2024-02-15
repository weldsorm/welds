use crate::model_traits::Column;
use crate::Syntax;

pub struct ColumnWriter {
    syntax: Syntax,
}
impl ColumnWriter {
    pub fn new(syntax: Syntax) -> Self {
        Self { syntax }
    }
    pub fn write(&self, prefix: &str, col: &Column) -> String {
        match self.syntax {
            #[cfg(feature = "mysql")]
            Syntax::Mysql => MySql::write(prefix, col),
            #[cfg(feature = "postgres")]
            Syntax::Postgres => Postgres::write(prefix, col),
            #[cfg(feature = "sqlite")]
            Syntax::Sqlite => Sqlite::write(prefix, col),
            #[cfg(feature = "mssql")]
            Syntax::Mssql => Mssql::write(prefix, col),
        }
    }
    pub fn excape(&self, name: &str) -> String {
        match self.syntax {
            #[cfg(feature = "mysql")]
            Syntax::Mysql => MySql::excape(name),
            #[cfg(feature = "postgres")]
            Syntax::Postgres => Postgres::excape(name),
            #[cfg(feature = "sqlite")]
            Syntax::Sqlite => Sqlite::excape(name),
            #[cfg(feature = "mssql")]
            Syntax::Mssql => Mssql::excape(name),
        }
    }
}

#[cfg(feature = "postgres")]
struct Postgres;

#[cfg(feature = "postgres")]
impl Postgres {
    fn excape(name: &str) -> String {
        format!("\"{}\"", name)
    }
    fn write(prefix: &str, col: &Column) -> String {
        let name = Self::excape(col.name());
        format!("{}.{}", prefix, name)
    }
}

#[cfg(feature = "sqlite")]
struct Sqlite;

#[cfg(feature = "sqlite")]
impl Sqlite {
    fn excape(name: &str) -> String {
        format!("\"{}\"", name)
    }
    fn write(prefix: &str, col: &Column) -> String {
        let name = Self::excape(col.name());
        format!("{}.{}", prefix, name)
    }
}

#[cfg(feature = "mysql")]
struct MySql;

#[cfg(feature = "mysql")]
impl MySql {
    fn excape(name: &str) -> String {
        name.to_string()
    }
    fn write(prefix: &str, col: &Column) -> String {
        let name = Self::excape(col.name());
        format!("{}.{}", prefix, name)
    }
}

#[cfg(feature = "mssql")]
struct Mssql;

#[cfg(feature = "mssql")]
impl Mssql {
    fn excape(name: &str) -> String {
        format!("\"{}\"", name)
    }
    fn write(prefix: &str, col: &Column) -> String {
        let name = Self::excape(col.name());
        format!("{}.{}", prefix, name)
    }
}

//fn mssql_type_overrides(dbtype: &str) -> &str {
//    match dbtype.to_lowercase().as_str() {
//        //
//        "bit" => "tinyint",
//        "bitn" => "tinyint",
//        _ => dbtype,
//    }
//}
