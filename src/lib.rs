pub mod commands;
mod errors;
pub mod generators;
pub mod schema;
pub mod adapters;

use errors::Result;
pub use errors::WeldsError;
use std::path::PathBuf;

pub async fn update(schema_path: PathBuf, table: Option<String>) -> Result<()> {
    println!(
        "UPDATE: {} {:?}",
        schema_path.to_string_lossy().to_string(),
        table
    );

    Ok(())
}

#[derive(Debug, Default)]
pub struct GenerateOption {
    pub schema_path: PathBuf,
    pub project_dir: PathBuf,
    pub table: Option<String>,
    pub force: bool,
}

pub fn generate(opt: GenerateOption) -> Result<()> {
    if !opt.schema_path.exists() {
        return Err(WeldsError::MissingSchemaFile(opt.schema_path));
    }

    let config = schema::read(&opt.schema_path)?;
    generators::models::run(&config, &opt)?;

    Ok(())
}
