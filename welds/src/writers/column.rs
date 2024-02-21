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
            Syntax::Mysql => MySql::write(prefix, col),
            Syntax::Postgres => Postgres::write(prefix, col),
            Syntax::Sqlite => Sqlite::write(prefix, col),
            Syntax::Mssql => Mssql::write(prefix, col),
        }
    }
    pub fn excape(&self, name: &str) -> String {
        match self.syntax {
            Syntax::Mysql => MySql::excape(name),
            Syntax::Postgres => Postgres::excape(name),
            Syntax::Sqlite => Sqlite::excape(name),
            Syntax::Mssql => Mssql::excape(name),
        }
    }
}

struct Postgres;
impl Postgres {
    fn excape(name: &str) -> String {
        format!("\"{}\"", name)
    }
    fn write(prefix: &str, col: &Column) -> String {
        let name = Self::excape(col.name());
        format!("{}.{}", prefix, name)
    }
}

struct Sqlite;
impl Sqlite {
    fn excape(name: &str) -> String {
        format!("\"{}\"", name)
    }
    fn write(prefix: &str, col: &Column) -> String {
        let name = Self::excape(col.name());
        format!("{}.{}", prefix, name)
    }
}

struct MySql;
impl MySql {
    fn excape(name: &str) -> String {
        name.to_string()
    }
    fn write(prefix: &str, col: &Column) -> String {
        let name = Self::excape(col.name());
        format!("{}.{}", prefix, name)
    }
}

struct Mssql;
impl Mssql {
    fn excape(name: &str) -> String {
        format!("\"{}\"", name)
    }
    fn write(prefix: &str, col: &Column) -> String {
        let name = Self::excape(col.name());
        format!("{}.{}", prefix, name)
    }
}
