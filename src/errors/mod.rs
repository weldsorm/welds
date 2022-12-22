use std::path::PathBuf;
pub(crate) type Result<T> = std::result::Result<T, WeldsError>;

#[derive(Debug)]
pub enum WeldsError {
    MissingSchemaFile(PathBuf),
    DbError(sqlx::Error),
}

impl std::error::Error for WeldsError {}

impl std::fmt::Display for WeldsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use WeldsError::*;

        match self {
            MissingSchemaFile(path) => {
                write!(
                    f,
                    "The schema definition file {} could not be found.",
                    path.to_string_lossy()
                )
            }
            DbError(_err) => write!(f, "(ServiceError)"),
        }
    }
}

impl From<sqlx::Error> for WeldsError {
    fn from(inner: sqlx::Error) -> WeldsError {
        WeldsError::DbError(inner)
    }
}
