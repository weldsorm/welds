pub mod commands;
mod errors;
pub mod generators;
pub mod schema;

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

pub async fn generate(schema_path: PathBuf, table: Option<String>) -> Result<()> {
    if !schema_path.exists() {
        return Err(WeldsError::MissingSchemaFile(schema_path));
    }

    let config = schema::read(&schema_path)?;
    //println!("CONFIG: {:?}", config);

    Ok(())
}
