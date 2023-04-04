use std::path::PathBuf;
pub type Result<T> = std::result::Result<T, WeldsError>;
use welds::errors::WeldsError as WeldsCoreError;

#[derive(Debug)]
pub enum WeldsError {
    MissingSchemaFile(PathBuf),
    ReadError(PathBuf),
    InvalidProject,
    IoError(std::io::Error),
    ConfigReadError(PathBuf),
    ConfigWrite,
    Core(WeldsCoreError),
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
            ConfigReadError(path) => write!(
                f,
                "There is an error in the config file {}. ",
                path.to_string_lossy()
            ),
            IoError(inner) => write!(f, "There was an IO error: {}", inner),
            InvalidProject => write!(
                f,
                "It doesn't appear you are working in a valid rust project."
            ),
            ConfigWrite => write!(f, "There was an unknown error writing the weld.yaml config"),
            Core(inner) => write!(f, "{}", inner),
        }
    }
}

impl From<sqlx::Error> for WeldsError {
    fn from(inner: sqlx::Error) -> WeldsError {
        let inin = WeldsCoreError::DbError(inner);
        WeldsError::Core(inin)
    }
}

impl From<std::io::Error> for WeldsError {
    fn from(inner: std::io::Error) -> WeldsError {
        WeldsError::IoError(inner)
    }
}

impl From<WeldsCoreError> for WeldsError {
    fn from(inner: WeldsCoreError) -> WeldsError {
        WeldsError::Core(inner)
    }
}
