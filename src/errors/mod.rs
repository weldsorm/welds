use std::path::PathBuf;
pub(crate) type Result<T> = std::result::Result<T, WeldsError>;

#[derive(Debug)]
pub enum WeldsError {
    MissingSchemaFile(PathBuf),
    ReadError(PathBuf),
    InvalidProject,
    IoError(std::io::Error),
    ConfigReadError((PathBuf, serde_yaml::Error)),
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
            ReadError(path) => write!(
                f,
                "There was an error reading the file {}",
                path.to_string_lossy()
            ),
            ConfigReadError((path, yaml_err)) => write!(
                f,
                "There is an error in the config file {}. \n {}",
                path.to_string_lossy(),
                yaml_err
            ),
            IoError(inner) => write!(f, "There was an IO error: {}", inner),
            InvalidProject => write!(
                f,
                "It doesn't appear you are working in a valid rust project."
            ),
            DbError(err) => write!(f, "{}", err),
        }
    }
}

impl From<sqlx::Error> for WeldsError {
    fn from(inner: sqlx::Error) -> WeldsError {
        WeldsError::DbError(inner)
    }
}

impl From<std::io::Error> for WeldsError {
    fn from(inner: std::io::Error) -> WeldsError {
        WeldsError::IoError(inner)
    }
}
