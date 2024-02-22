use super::table_def::DataType;
use crate::model_traits::TableIdent;
use crate::Row;

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

impl TryFrom<Row> for TableScanRow {
    type Error = crate::WeldsError;
    fn try_from(row: Row) -> Result<Self, Self::Error> {
        Ok(TableScanRow {
            schema: row.get_by_position(0)?,
            table_name: row.get_by_position(1)?,
            ty: row.get_by_position(2)?,
            column_name: row.get_by_position(3)?,
            column_type: row.get_by_position(4)?,
            is_nullable: row.get_by_position(5)?,
            is_primary_key: row.get_by_position(6)?,
            is_updatable: row.get_by_position(7)?,
        })
    }
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
