use crate::Row;
use crate::model_traits::TableIdent;

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

impl TryFrom<Row> for FkScanRow {
    type Error = crate::errors::WeldsError;
    fn try_from(row: Row) -> Result<Self, Self::Error> {
        let c0 = row.get_by_position(0)?;
        let c1 = row.get_by_position(1)?;
        let c2 = row.get_by_position(2)?;
        let c3 = row.get_by_position(3)?;
        let c4 = row.get_by_position(4)?;
        let c5 = row.get_by_position(5)?;
        Ok(FkScanRow {
            me: FkScanTableCol::new(c0, c1, c2),
            other: FkScanTableCol::new(c3, c4, c5),
        })
    }
}
