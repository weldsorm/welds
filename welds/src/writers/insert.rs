pub struct ColArg(pub String, pub String);
use super::column::ColumnWriter;
use crate::table::Column;

type Sql = (String, Option<String>);

pub(crate) struct InsertWriter {
    write: fn(identifier: &str, &[ColArg], &[Column], &[Column]) -> Sql,
}

impl InsertWriter {
    pub fn new<DB: DbInsertWriter>() -> Self {
        Self { write: DB::write }
    }

    pub fn write(
        &self,
        identifier: &str,
        colargs: &[ColArg],
        columns: &[Column],
        pks: &[Column],
    ) -> Sql {
        (self.write)(identifier, colargs, columns, pks)
    }
}

pub trait DbInsertWriter {
    fn write(identifier: &str, colargs: &[ColArg], columns: &[Column], pks: &[Column]) -> Sql;
}

#[cfg(feature = "postgres")]
impl DbInsertWriter for sqlx::Postgres {
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

#[cfg(feature = "sqlite")]
impl DbInsertWriter for sqlx::Sqlite {
    fn write(identifier: &str, colargs: &[ColArg], _columns: &[Column], pks: &[Column]) -> Sql {
        assert!(
            pks.len() == 1,
            "Error: A single primary key is required for insert"
        );
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

#[cfg(feature = "mysql")]
impl DbInsertWriter for sqlx::MySql {
    fn write(identifier: &str, colargs: &[ColArg], _columns: &[Column], pks: &[Column]) -> Sql {
        assert!(
            pks.len() == 1,
            "Error: A single primary key is required for insert"
        );
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

#[cfg(feature = "mssql")]
impl DbInsertWriter for sqlx::Mssql {
    fn write(identifier: &str, colargs: &[ColArg], columns: &[Column], _pks: &[Column]) -> Sql {
        let cols: Vec<_> = colargs.iter().map(|x| x.0.as_str()).collect();
        let args: Vec<_> = colargs.iter().map(|x| x.1.as_str()).collect();
        let col_group = cols.join(", ");
        let arg_group = args.join(", ");

        // write the column select that will be returned
        let col_write = ColumnWriter::new::<sqlx::Mssql>();
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
