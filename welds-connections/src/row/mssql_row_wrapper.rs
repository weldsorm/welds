use crate::errors::Error;
use crate::errors::Result;
use tiberius::Column;
use tiberius::Row as MssqlRow;

pub struct MssqlRowWrapper {
    cells: Vec<Cell>,
}

pub struct Cell {
    column: Column,
    data: ColumnData<'static>,
}

impl Cell {
    pub fn column(&self) -> &Column {
        &self.column
    }
    pub fn data(&self) -> &ColumnData<'static> {
        &self.data
    }
}

impl MssqlRowWrapper {
    pub(crate) fn new(row: MssqlRow) -> MssqlRowWrapper {
        let mut columns = row.columns().to_vec();
        let datas = row.into_iter();
        let cells: Vec<Cell> = datas
            .zip(columns.drain(..))
            .map(|(data, column)| Cell {
                data,
                column: column.clone(),
            })
            .collect();
        Self { cells }
    }

    /// Returns a slice of the cells that make up this row
    pub fn cells(&self) -> &[Cell] {
        &self.cells
    }

    /// Returns an owned version of the cells that make up this row
    pub fn into_inner(self) -> Vec<Cell> {
        self.cells
    }

    /// returns true is the column is in the row, find by name
    pub fn has_column(&self, name: &str) -> bool {
        for cell in &self.cells {
            if cell.column.name() == name {
                return true;
            }
        }
        false
    }

    /// returns true is the column is in the row, find by index
    pub fn has_index(&self, index: usize) -> bool {
        self.cells.len() > index
    }

    /// Try and fetch the data from the row/column into a type <T>
    /// Errors
    ///  * if the column name is not in row
    ///  * the cell can not be deserialized into <T>
    pub fn try_get<T>(&self, name: &str) -> Result<T>
    where
        T: TiberiusDecode,
    {
        for cell in &self.cells {
            if cell.column.name() == name {
                return TiberiusDecode::read(cell.column(), cell.data.clone());
            }
        }
        Err(Error::ColumnNotFound(name.to_owned()))
    }

    /// Try and fetch the data from the row/column into a type <T>
    /// Errors
    ///  * if the column index is not in row
    ///  * the cell can not be deserialized into <T>
    pub fn try_get_by_posision<T>(&self, idx: usize) -> Result<T>
    where
        T: TiberiusDecode,
    {
        let cell: &Cell = self
            .cells
            .get(idx)
            .ok_or_else(|| Error::ColumnNotFound(format!("BY_INDEX: {}", idx)))?;
        TiberiusDecode::read(cell.column(), cell.data.clone())
    }
}

use tiberius::ColumnData;
use tiberius::FromSqlOwned;

pub trait TiberiusDecode
where
    Self: Sized,
{
    fn read(col: &Column, value: ColumnData<'static>) -> Result<Self>;
}

impl<T> TiberiusDecode for Option<T>
where
    T: FromSqlOwned,
{
    fn read(_col: &Column, value: ColumnData<'static>) -> Result<Self> {
        Ok(FromSqlOwned::from_sql_owned(value)?)
    }
}

impl TiberiusDecode for String {
    fn read(col: &Column, value: ColumnData<'static>) -> Result<Self> {
        let v = FromSqlOwned::from_sql_owned(value)?;
        v.ok_or_else(|| Error::UnexpectedNoneInColumn(col.name().to_owned()))
    }
}

impl TiberiusDecode for Vec<u8> {
    fn read(col: &Column, value: ColumnData<'static>) -> Result<Self> {
        let v = FromSqlOwned::from_sql_owned(value)?;
        v.ok_or_else(|| Error::UnexpectedNoneInColumn(col.name().to_owned()))
    }
}

impl TiberiusDecode for bool {
    fn read(col: &Column, value: ColumnData<'static>) -> Result<Self> {
        let v = FromSqlOwned::from_sql_owned(value)?;
        v.ok_or_else(|| Error::UnexpectedNoneInColumn(col.name().to_owned()))
    }
}

impl TiberiusDecode for f32 {
    fn read(col: &Column, value: ColumnData<'static>) -> Result<Self> {
        let v = FromSqlOwned::from_sql_owned(value)?;
        v.ok_or_else(|| Error::UnexpectedNoneInColumn(col.name().to_owned()))
    }
}

impl TiberiusDecode for f64 {
    fn read(col: &Column, value: ColumnData<'static>) -> Result<Self> {
        let v = FromSqlOwned::from_sql_owned(value)?;
        v.ok_or_else(|| Error::UnexpectedNoneInColumn(col.name().to_owned()))
    }
}

impl TiberiusDecode for i16 {
    fn read(col: &Column, value: ColumnData<'static>) -> Result<Self> {
        let v = FromSqlOwned::from_sql_owned(value)?;
        v.ok_or_else(|| Error::UnexpectedNoneInColumn(col.name().to_owned()))
    }
}

impl TiberiusDecode for i32 {
    fn read(col: &Column, value: ColumnData<'static>) -> Result<Self> {
        let v = FromSqlOwned::from_sql_owned(value)?;
        v.ok_or_else(|| Error::UnexpectedNoneInColumn(col.name().to_owned()))
    }
}

impl TiberiusDecode for i64 {
    fn read(col: &Column, value: ColumnData<'static>) -> Result<Self> {
        let v = FromSqlOwned::from_sql_owned(value)?;
        v.ok_or_else(|| Error::UnexpectedNoneInColumn(col.name().to_owned()))
    }
}

impl TiberiusDecode for u8 {
    fn read(col: &Column, value: ColumnData<'static>) -> Result<Self> {
        let v = FromSqlOwned::from_sql_owned(value)?;
        v.ok_or_else(|| Error::UnexpectedNoneInColumn(col.name().to_owned()))
    }
}

impl TiberiusDecode for tiberius::Uuid {
    fn read(col: &Column, value: ColumnData<'static>) -> Result<Self> {
        let v = FromSqlOwned::from_sql_owned(value)?;
        v.ok_or_else(|| Error::UnexpectedNoneInColumn(col.name().to_owned()))
    }
}

impl TiberiusDecode for tiberius::xml::XmlData {
    fn read(col: &Column, value: ColumnData<'static>) -> Result<Self> {
        let v = FromSqlOwned::from_sql_owned(value)?;
        v.ok_or_else(|| Error::UnexpectedNoneInColumn(col.name().to_owned()))
    }
}

impl TiberiusDecode for tiberius::numeric::Numeric {
    fn read(col: &Column, value: ColumnData<'static>) -> Result<Self> {
        let v = FromSqlOwned::from_sql_owned(value)?;
        v.ok_or_else(|| Error::UnexpectedNoneInColumn(col.name().to_owned()))
    }
}
#[cfg(feature = "mssql-rust_decimal")]
impl TiberiusDecode for tiberius::numeric::Decimal {
    fn read(col: &Column, value: ColumnData<'static>) -> Result<Self> {
        let v = FromSqlOwned::from_sql_owned(value)?;
        v.ok_or_else(|| Error::UnexpectedNoneInColumn(col.name().to_owned()))
    }
}
#[cfg(feature = "mssql-bigdecimal")]
impl TiberiusDecode for tiberius::numeric::BigDecimal {
    fn read(col: &Column, value: ColumnData<'static>) -> Result<Self> {
        let v = FromSqlOwned::from_sql_owned(value)?;
        v.ok_or_else(|| Error::UnexpectedNoneInColumn(col.name().to_owned()))
    }
}

#[cfg(feature = "mssql-chrono")]
/// All mapping for the chrono crate
mod chrono {
    use super::*;
    impl TiberiusDecode for tiberius::time::chrono::NaiveDateTime {
        fn read(col: &Column, value: ColumnData<'static>) -> Result<Self> {
            let v = FromSqlOwned::from_sql_owned(value)?;
            v.ok_or_else(|| Error::UnexpectedNoneInColumn(col.name().to_owned()))
        }
    }
    impl TiberiusDecode for tiberius::time::chrono::NaiveDate {
        fn read(col: &Column, value: ColumnData<'static>) -> Result<Self> {
            let v = FromSqlOwned::from_sql_owned(value)?;
            v.ok_or_else(|| Error::UnexpectedNoneInColumn(col.name().to_owned()))
        }
    }
    impl TiberiusDecode for tiberius::time::chrono::NaiveTime {
        fn read(col: &Column, value: ColumnData<'static>) -> Result<Self> {
            let v = FromSqlOwned::from_sql_owned(value)?;
            v.ok_or_else(|| Error::UnexpectedNoneInColumn(col.name().to_owned()))
        }
    }
    impl TiberiusDecode for tiberius::time::chrono::DateTime<tiberius::time::chrono::Utc> {
        fn read(col: &Column, value: ColumnData<'static>) -> Result<Self> {
            let v = FromSqlOwned::from_sql_owned(value)?;
            v.ok_or_else(|| Error::UnexpectedNoneInColumn(col.name().to_owned()))
        }
    }
    impl TiberiusDecode for tiberius::time::chrono::DateTime<tiberius::time::chrono::FixedOffset> {
        fn read(col: &Column, value: ColumnData<'static>) -> Result<Self> {
            let v = FromSqlOwned::from_sql_owned(value)?;
            v.ok_or_else(|| Error::UnexpectedNoneInColumn(col.name().to_owned()))
        }
    }
}

#[cfg(feature = "mssql-time")]
/// All mapping for the time crate
mod time {
    use super::*;

    impl TiberiusDecode for tiberius::time::time::Date {
        fn read(col: &Column, value: ColumnData<'static>) -> Result<Self> {
            let v = FromSqlOwned::from_sql_owned(value)?;
            v.ok_or_else(|| Error::UnexpectedNoneInColumn(col.name().to_owned()))
        }
    }
    impl TiberiusDecode for tiberius::time::time::Time {
        fn read(col: &Column, value: ColumnData<'static>) -> Result<Self> {
            let v = FromSqlOwned::from_sql_owned(value)?;
            v.ok_or_else(|| Error::UnexpectedNoneInColumn(col.name().to_owned()))
        }
    }
    impl TiberiusDecode for tiberius::time::time::OffsetDateTime {
        fn read(col: &Column, value: ColumnData<'static>) -> Result<Self> {
            let v = FromSqlOwned::from_sql_owned(value)?;
            v.ok_or_else(|| Error::UnexpectedNoneInColumn(col.name().to_owned()))
        }
    }
    impl TiberiusDecode for tiberius::time::time::PrimitiveDateTime {
        fn read(col: &Column, value: ColumnData<'static>) -> Result<Self> {
            let v = FromSqlOwned::from_sql_owned(value)?;
            v.ok_or_else(|| Error::UnexpectedNoneInColumn(col.name().to_owned()))
        }
    }
}
