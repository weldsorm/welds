use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    #[cfg(any(feature = "mysql", feature = "sqlite", feature = "postgres"))]
    Sqlx(sqlx::Error),
    #[cfg(feature = "mssql")]
    TiberiusConnPool(bb8_tiberius::Error),
    #[cfg(feature = "mssql")]
    Tiberius(tiberius::error::Error),
    Bb8(&'static str),
    InvalidDatabaseUrl,
    RowNowFound,
    ColumnNotFound(String),
    UnexpectedNoneInColumn(String),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Bad Thing")?;
        Ok(())
    }
}

#[cfg(any(feature = "mysql", feature = "sqlite", feature = "postgres"))]
impl From<sqlx::error::Error> for Error {
    fn from(inner: sqlx::error::Error) -> Self {
        Error::Sqlx(inner)
    }
}

#[cfg(feature = "mssql")]
impl From<bb8_tiberius::Error> for Error {
    fn from(inner: bb8_tiberius::Error) -> Self {
        Error::TiberiusConnPool(inner)
    }
}

#[cfg(feature = "mssql")]
impl<T> From<bb8::RunError<T>> for Error {
    fn from(inner: bb8::RunError<T>) -> Self {
        let inner = match inner {
            bb8::RunError::TimedOut => "bb8 timeout",
            bb8::RunError::User(_) => "bb8 user error",
        };
        Error::Bb8(inner)
    }
}

#[cfg(feature = "mssql")]
impl From<tiberius::error::Error> for Error {
    fn from(inner: tiberius::error::Error) -> Self {
        Error::Tiberius(inner)
    }
}
