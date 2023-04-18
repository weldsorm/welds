pub type Result<T> = std::result::Result<T, WeldsError>;

#[derive(Debug)]
pub enum WeldsError {
    DbError(sqlx::Error),
    InsertFailed(String),
    NoDatabaseUrl,
    UnsupportedDatabase,
    MissingDbColumn(String),
    NoPrimaryKey,
}

impl std::error::Error for WeldsError {}

impl std::fmt::Display for WeldsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use WeldsError::*;

        match self {
            DbError(err) => write!(f, "{}", err),
            NoDatabaseUrl => write!(f, "`DATABASE_URL` must be set to use welds"),
            MissingDbColumn(c) => write!(f, "The Database column is not present: {}", c),
            NoPrimaryKey => write!(f, "This Action can not be preformed without a primary key"),
            InsertFailed(s) => write!(f, "The create of Obj in the database failed: {}", s),
            UnsupportedDatabase => write!(
                f,
                "`DATABASE_URL` does not contain a URL to a supported Database"
            ),
        }
    }
}

impl From<sqlx::Error> for WeldsError {
    fn from(inner: sqlx::Error) -> WeldsError {
        WeldsError::DbError(inner)
    }
}
