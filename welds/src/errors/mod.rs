use thiserror::Error;
use welds_connections::Error as ConnError;

pub type Result<T> = std::result::Result<T, WeldsError>;

#[derive(Error, Debug)]
pub enum WeldsError {
    #[error("An Error From the Database: {0}")]
    Database(ConnError),
    //#[cfg(any(feature = "mysql", feature = "sqlite", feature = "postgres"))]
    //Sqlx(sqlx::Error),
    //#[cfg(feature = "mssql")]
    //TiberiusConnPool(bb8_tiberius::Error),
    //#[cfg(feature = "mssql")]
    //Tiberius(tiberius::error::Error),
    //Bb8(&'static str),
    //InvalidDatabaseUrl,
    #[error("Failed to Insert {0}")]
    InsertFailed(String),
    #[error("Expected Row from DB, Found none")]
    RowNowFound,
    #[error("A Primary key is required for this action")]
    NoPrimaryKey, //ColumnNotFound(String),
                  //UnexpectedNoneInColumn(String),
}

impl From<ConnError> for WeldsError {
    fn from(inner: ConnError) -> Self {
        WeldsError::Database(inner)
    }
}
