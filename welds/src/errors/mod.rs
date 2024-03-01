use crate::model_traits::TableIdent;
use thiserror::Error;
use welds_connections::Error as ConnError;

pub type Result<T> = std::result::Result<T, WeldsError>;

#[derive(Error, Debug)]
pub enum WeldsError {
    #[error("An Error From the Database: {0}")]
    Database(ConnError),
    #[error("Could not find tablebase table {0}")]
    MissingTable(TableIdent),
    #[error("The Database column is not present: {0}")]
    MigrationError(String),
    #[error("Migration Error: {0}")]
    MissingDbColumn(String),
    #[error("Failed to Insert {0}")]
    InsertFailed(String),
    #[error("Expected Row from DB, Found none")]
    RowNowFound,
    #[error("A Primary key is required for this action")]
    NoPrimaryKey,
    #[error("There are multiple migrations with the same name")]
    DuplicateMigration,
}

impl From<ConnError> for WeldsError {
    fn from(inner: ConnError) -> Self {
        WeldsError::Database(inner)
    }
}
