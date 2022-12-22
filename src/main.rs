use std::path::PathBuf;

use sqlx::Connection;
use sqlx::SqliteConnection;
mod commands;
mod schema;
use clap::Parser;

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    let args = commands::Args::parse();

    let schema_file = args
        .schema_file
        .clone()
        .unwrap_or_else(|| PathBuf::from("./welds.yaml"));

    let schema_text = std::fs::read_to_string(schema_file.as_path())?;
    let config: Result<schema::Config, _> = serde_yaml::from_str(&schema_text);

    println!("PATH: {:?}", config);

    //let _pool = SqliteConnection::connect("sqlite::memory:").await?;
    Ok(())
}
