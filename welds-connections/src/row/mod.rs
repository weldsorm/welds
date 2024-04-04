use crate::errors::Result;

/// A row of data from the database
pub struct Row {
    inner: RowInner,
}

#[cfg(feature = "mysql")]
use sqlx::mysql::MySqlRow;
#[cfg(feature = "postgres")]
use sqlx::postgres::PgRow;
#[cfg(feature = "sqlite")]
use sqlx::sqlite::SqliteRow;
#[cfg(any(feature = "mysql", feature = "sqlite", feature = "postgres"))]
use sqlx::Decode;
#[cfg(any(feature = "mysql", feature = "sqlite", feature = "postgres"))]
use sqlx::Type;

#[cfg(feature = "mssql")]
use tiberius::Row as MssqlRow;

#[cfg(feature = "mssql")]
mod mssql_row_wrapper;
#[cfg(feature = "mssql")]
use mssql_row_wrapper::MssqlRowWrapper;

/// all kinds of rows
pub enum RowInner {
    #[cfg(feature = "sqlite")]
    Sqlite(SqliteRow),
    #[cfg(feature = "mssql")]
    Mssql(MssqlRowWrapper),
    #[cfg(feature = "postgres")]
    Postgres(PgRow),
    #[cfg(feature = "mysql")]
    Mysql(MySqlRow),
}

#[cfg(feature = "sqlite")]
impl From<SqliteRow> for Row {
    fn from(r: SqliteRow) -> Row {
        Row {
            inner: RowInner::Sqlite(r),
        }
    }
}

#[cfg(feature = "mssql")]
impl From<MssqlRow> for Row {
    fn from(r: MssqlRow) -> Row {
        Row {
            inner: RowInner::Mssql(MssqlRowWrapper::new(r)),
        }
    }
}

#[cfg(feature = "postgres")]
impl From<PgRow> for Row {
    fn from(r: PgRow) -> Row {
        Row {
            inner: RowInner::Postgres(r),
        }
    }
}

#[cfg(feature = "mysql")]
impl From<MySqlRow> for Row {
    fn from(r: MySqlRow) -> Row {
        Row {
            inner: RowInner::Mysql(r),
        }
    }
}

#[cfg(feature = "sqlite")]
impl Row {
    pub fn as_sqlite_row(self) -> Option<SqliteRow> {
        match self.inner {
            RowInner::Sqlite(r) => Some(r),
            _ => None,
        }
    }
}

#[cfg(feature = "postgres")]
impl Row {
    pub fn as_postgres_row(self) -> Option<PgRow> {
        match self.inner {
            RowInner::Postgres(r) => Some(r),
            _ => None,
        }
    }
}

#[cfg(feature = "mysql")]
impl Row {
    pub fn as_mysql_row(self) -> Option<MySqlRow> {
        match self.inner {
            RowInner::Mysql(r) => Some(r),
            _ => None,
        }
    }
}

#[cfg(feature = "mssql")]
impl Row {
    pub fn as_mssql_row(self) -> Option<MssqlRowWrapper> {
        match self.inner {
            RowInner::Mssql(r) => Some(r),
            _ => None,
        }
    }
}

#[cfg(feature = "mssql")]
use mssql_row_wrapper::TiberiusDecode;

// This code is scripted out cuz writing it for all the features to be to much
mod row_gen;
