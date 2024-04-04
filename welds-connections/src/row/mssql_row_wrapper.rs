use crate::errors::Error;
use crate::errors::Result;
use tiberius::Column;
use tiberius::Row as MssqlRow;

pub struct MssqlRowWrapper {
    columns: Vec<Column>,
    datas: Vec<ColumnData<'static>>,
}

impl MssqlRowWrapper {
    pub(crate) fn new(row: MssqlRow) -> MssqlRowWrapper {
        Self {
            columns: row.columns().to_vec(),
            datas: row.into_iter().collect(),
        }
    }

    pub fn try_get<T>(&self, name: &str) -> Result<T>
    where
        T: TiberiusDecode,
    {
        for (col, data) in self.columns.iter().zip(self.datas.iter()) {
            if col.name() == name {
                return TiberiusDecode::read(col, data.clone());
            }
        }
        Err(Error::ColumnNotFound(name.to_owned()))
    }

    pub fn try_get_by_posision<T>(&self, idx: usize) -> Result<T>
    where
        T: TiberiusDecode,
    {
        let data: &ColumnData = self
            .datas
            .get(idx)
            .ok_or_else(|| Error::ColumnNotFound(format!("BY_INDEX: {}", idx)))?;
        let col: &Column = self
            .columns
            .get(idx)
            .ok_or_else(|| Error::ColumnNotFound(format!("BY_INDEX: {}", idx)))?;
        TiberiusDecode::read(col, data.clone())
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
    fn read(col: &Column, value: ColumnData<'static>) -> Result<Self> {
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

impl TiberiusDecode for tiberius::numeric::Numeric {
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
