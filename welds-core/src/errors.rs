pub type Result<T> = std::result::Result<T, WeldsError>;

#[derive(Debug)]
pub enum WeldsError {
    DbError(sqlx::Error),
    NoDatabaseUrl,
    UnsupportedDatabase,
}

impl std::error::Error for WeldsError {}

impl std::fmt::Display for WeldsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use WeldsError::*;

        match self {
            DbError(err) => write!(f, "{}", err),
            NoDatabaseUrl => write!(f, "`DATABASE_URL` must be set to use welds"),
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
