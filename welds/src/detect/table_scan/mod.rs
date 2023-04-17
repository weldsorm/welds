use crate::table::{DataType, TableIdent};

#[derive(Debug, sqlx::FromRow)]
pub struct TableScanRow {
    pub(super) schema: Option<String>,
    pub(super) table_name: String,
    pub(super) ty: String,
    pub(super) column_name: String,
    pub(super) column_type: String,
    pub(super) is_nullable: i32,
    pub(super) is_primary_key: i32,
    pub(super) is_updatable: i32,
}

impl TableScanRow {
    pub fn ident(&self) -> TableIdent {
        TableIdent {
            schema: self.schema.clone(),
            name: self.table_name.clone(),
        }
    }
    pub fn kind(&self) -> DataType {
        if self.ty == "table" {
            return DataType::Table;
        }
        DataType::View
    }
}

#[derive(Debug)]
pub struct FkScanRow {
    pub(super) me: FkScanTableCol,
    pub(super) other: FkScanTableCol,
}

#[derive(Debug)]
pub struct FkScanTableCol {
    pub(super) ident: TableIdent,
    pub(super) column: String,
}

impl FkScanTableCol {
    pub(super) fn new(schema: Option<String>, table: String, column: String) -> Self {
        Self {
            ident: TableIdent {
                schema,
                name: table,
            },
            column,
        }
    }
}

pub trait TableScan {
    /// returns the sql needed to get a list of table in the database
    /// a unique list is build from all the sql commands provided
    fn table_scan_sql() -> &'static str;
    fn fk_scan_sql() -> &'static str;
}

#[cfg(feature = "postgres")]
impl TableScan for sqlx::Postgres {
    fn table_scan_sql() -> &'static str {
        include_str!("./postgres.sql")
    }
    fn fk_scan_sql() -> &'static str {
        include_str!("./postgres_fk.sql")
    }
}

#[cfg(feature = "mysql")]
impl TableScan for sqlx::MySql {
    fn table_scan_sql() -> &'static str {
        include_str!("./mysql.sql")
    }
    fn fk_scan_sql() -> &'static str {
        include_str!("./mysql_fk.sql")
    }
}

#[cfg(feature = "mssql")]
impl TableScan for sqlx::Mssql {
    fn table_scan_sql() -> &'static str {
        include_str!("./mssql.sql")
    }
    fn fk_scan_sql() -> &'static str {
        include_str!("./mssql_fk.sql")
    }
}

#[cfg(feature = "sqlite")]
impl TableScan for sqlx::Sqlite {
    fn table_scan_sql() -> &'static str {
        include_str!("./sqlite.sql")
    }
    fn fk_scan_sql() -> &'static str {
        include_str!("./sqlite_fk.sql")
    }
}
