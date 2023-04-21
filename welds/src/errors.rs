use thiserror::Error;

#[derive(Debug, Error)]
pub enum WeldsError {
    #[error(transparent)]
    DbError(#[from] sqlx::Error),
    #[error("This create of Obj in the database failed: {0}")]
    InsertFailed(String),
    #[error("`DATABASE_URL` must be set to use welds")]
    NoDatabaseUrl,
    #[error("`DATABASE_URL` does not contain a URL to a supported Database")]
    UnsupportedDatabase,
    #[error("The Database column is not present: {0}")]
    MissingDbColumn(String),
    #[error("This Action can not be preformed without a primary key")]
    NoPrimaryKey,
}
