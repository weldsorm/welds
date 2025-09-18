pub struct ColArg(pub String, pub String);
use super::column::ColumnWriter;
use crate::Syntax;
use crate::model_traits::Column;

type Sql = (String, Option<String>);

pub struct InsertWriter {
    syntax: Syntax,
}

impl InsertWriter {
    pub fn new(syntax: Syntax) -> Self {
        Self { syntax }
    }
    pub fn write(
        &self,
        identifier: &str,
        colargs: &[ColArg],
        columns: &[Column],
        pks: &[Column],
    ) -> Sql {
        match self.syntax {
            Syntax::Mysql => MySql::write(identifier, colargs, columns, pks),
            Syntax::Postgres => Postgres::write(identifier, colargs, columns, pks),
            Syntax::Sqlite => Sqlite::write(identifier, colargs, columns, pks),
            Syntax::Mssql => Mssql::write(identifier, colargs, columns, pks),
        }
    }
}

struct Postgres;

impl Postgres {
    fn write(identifier: &str, colargs: &[ColArg], _columns: &[Column], _pks: &[Column]) -> Sql {
        let cols: Vec<_> = colargs.iter().map(|x| x.0.as_str()).collect();
        let args: Vec<_> = colargs.iter().map(|x| x.1.as_str()).collect();
        let col_group = cols.join(", ");
        let arg_group = args.join(", ");
        (
            format!(
                "INSERT INTO {} ({}) VALUES ({}) RETURNING *",
                identifier, col_group, arg_group
            ),
            None,
        )
    }
}

struct Sqlite;

impl Sqlite {
    fn write(identifier: &str, colargs: &[ColArg], _columns: &[Column], pks: &[Column]) -> Sql {
        let cols: Vec<_> = colargs.iter().map(|x| x.0.as_str()).collect();
        let args: Vec<_> = colargs.iter().map(|x| x.1.as_str()).collect();
        let col_group = cols.join(", ");
        let arg_group = args.join(", ");
        let insert = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            identifier, col_group, arg_group
        );
        let fetch = format!(
            "SELECT * FROM {} where {}=last_insert_rowid()",
            identifier,
            pks[0].name()
        );
        (insert, Some(fetch))
    }
}

struct MySql;

impl MySql {
    fn write(identifier: &str, colargs: &[ColArg], _columns: &[Column], pks: &[Column]) -> Sql {
        let cols: Vec<_> = colargs.iter().map(|x| x.0.as_str()).collect();
        let args: Vec<_> = colargs.iter().map(|x| x.1.as_str()).collect();
        let col_group = cols.join(", ");
        let arg_group = args.join(", ");
        let insert = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            identifier, col_group, arg_group
        );
        let fetch = format!(
            "SELECT * FROM {} where {}=LAST_INSERT_ID()",
            identifier,
            pks[0].name()
        );
        (insert, Some(fetch))
    }
}

struct Mssql;

impl Mssql {
    fn write(identifier: &str, colargs: &[ColArg], columns: &[Column], _pks: &[Column]) -> Sql {
        let cols: Vec<_> = colargs.iter().map(|x| x.0.as_str()).collect();
        let args: Vec<_> = colargs.iter().map(|x| x.1.as_str()).collect();
        let col_group = cols.join(", ");
        let arg_group = args.join(", ");

        // write the column select that will be returned
        let col_write = ColumnWriter::new(Syntax::Mssql);
        let return_col: Vec<String> = columns
            .iter()
            .map(|c| col_write.write("Inserted", c))
            .collect();
        let outputs = return_col.join(", ");

        (
            format!(
                "INSERT INTO {} ({}) OUTPUT {} VALUES ({})",
                identifier, col_group, outputs, arg_group
            ),
            None,
        )
    }
}
