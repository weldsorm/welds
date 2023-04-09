use crate::table::{DataType, RelationDef, TableIdent};

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
        return DataType::View;
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct FkScanRow {
    pub(super) me: RelationDef,
    pub(super) other: RelationDef,
}

pub trait TableScan {
    /// returns the sql needed to get a list of table in the database
    /// a unique list is build from all the sql commands provided
    fn table_scan_sql() -> &'static str;
    fn fk_scan_sql() -> &'static str;
}

impl TableScan for sqlx::Postgres {
    fn table_scan_sql() -> &'static str {
        include_str!("./postgres.sql")
    }
    fn fk_scan_sql() -> &'static str {
        include_str!("./postgres_fk.sql")
    }
}

impl TableScan for sqlx::MySql {
    fn table_scan_sql() -> &'static str {
        include_str!("./mysql.sql")
    }
    fn fk_scan_sql() -> &'static str {
        include_str!("./mysql_fk.sql")
    }
}

impl TableScan for sqlx::Mssql {
    fn table_scan_sql() -> &'static str {
        include_str!("./mssql.sql")
    }
    fn fk_scan_sql() -> &'static str {
        include_str!("./mssql_fk.sql")
    }
}

impl TableScan for sqlx::Sqlite {
    fn table_scan_sql() -> &'static str {
        include_str!("./sqlite.sql")
    }
    fn fk_scan_sql() -> &'static str {
        include_str!("./sqlite_fk.sql")
    }
}
