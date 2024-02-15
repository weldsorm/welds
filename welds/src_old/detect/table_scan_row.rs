use super::table_def::DataType;
use crate::table::TableIdent;
use sqlx::Row;

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

impl<R, DB> From<R> for TableScanRow
where
    DB: sqlx::Database,
    R: Row<Database = DB>,
    usize: sqlx::ColumnIndex<R>,
    i32: sqlx::Type<DB> + for<'r> sqlx::Decode<'r, DB>,
    String: sqlx::Type<DB> + for<'r> sqlx::Decode<'r, DB>,
    Option<String>: sqlx::Type<DB> + for<'r> sqlx::Decode<'r, DB>,
{
    fn from(r: R) -> Self {
        TableScanRow {
            schema: r.get(0),
            table_name: r.get(1),
            ty: r.get(2),
            column_name: r.get(3),
            column_type: r.get(4),
            is_nullable: r.get(5),
            is_primary_key: r.get(6),
            is_updatable: r.get(7),
        }
    }
}
