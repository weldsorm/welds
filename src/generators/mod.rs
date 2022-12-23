use crate::errors::{Result, WeldsError};
use std::path::PathBuf;

pub(crate) mod models;

fn validate_project_path(path: &PathBuf) -> Result<()> {
    if !path.exists() || !path.is_dir() {
        return Err(WeldsError::InvalidProject);
    }

    let mut src = PathBuf::from(path);
    src.push("src");
    if !src.exists() || !src.is_dir() {
        return Err(WeldsError::InvalidProject);
    }

    let mut cargo_toml = PathBuf::from(path);
    cargo_toml.push("Cargo.toml");
    if !cargo_toml.exists() || !cargo_toml.is_file() {
        return Err(WeldsError::InvalidProject);
    }

    // we could run cargo check...

    Ok(())
}
