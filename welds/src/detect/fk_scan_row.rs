use crate::table::TableIdent;
use sqlx::Row;

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

impl<R, DB> From<R> for FkScanRow
where
    DB: sqlx::Database,
    R: Row<Database = DB>,
    usize: sqlx::ColumnIndex<R>,
    i32: sqlx::Type<DB> + for<'r> sqlx::Decode<'r, DB>,
    String: sqlx::Type<DB> + for<'r> sqlx::Decode<'r, DB>,
    Option<String>: sqlx::Type<DB> + for<'r> sqlx::Decode<'r, DB>,
{
    fn from(r: R) -> Self {
        FkScanRow {
            me: FkScanTableCol::new(r.get(0), r.get(1), r.get(2)),
            other: FkScanTableCol::new(r.get(3), r.get(4), r.get(5)),
        }
    }
}
