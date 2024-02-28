pub mod commands;
pub mod config;
pub mod errors;
pub mod generators;

use anyhow::anyhow;
use anyhow::Result;
use config::DbProvider;
use std::path::PathBuf;
use welds::model_traits::TableIdent;
use welds::prelude::*;

pub async fn update(schema_path: PathBuf, identifier: Option<String>) -> Result<()> {
    use welds::detect::find_tables;
    let identifier = identifier.as_ref().map(|x| TableIdent::parse(x));

    let client = welds::connections::connect_from_env().await?;
    let mut tables = find_tables(client.as_ref()).await?;
    let provider: DbProvider = client.syntax().into();

    let mut conf_def = config::read(&schema_path).unwrap_or_default();

    match identifier {
        Some(identifier) => {
            // update only the specified table
            tables.retain(|t| t.ident() == &identifier);
            if tables.is_empty() {
                log::error!(
                    "Table not found: no table updated  (HINT: make sure you include the schema)"
                );
            }
            conf_def.add_update(provider, &tables);
        }
        None => {
            conf_def.remove_missing(&tables);
            conf_def.add_update(provider, &tables);
        }
    };

    config::write(&schema_path, &conf_def)?;
    Ok(())
}

#[derive(Debug, Default)]
pub struct GenerateOption {
    pub schema_path: PathBuf,
    pub output_path: PathBuf,
    pub hide_unknown_types: bool,
    pub table: Option<String>,
}

pub fn generate(mut opt: GenerateOption) -> Result<()> {
    use crate::errors::WeldsError;
    if !opt.schema_path.exists() {
        return Err(anyhow!(WeldsError::MissingSchemaFile(opt.schema_path)));
    }

    let conf_def = config::read(&opt.schema_path)?;

    clean_code_output_path(&mut opt);
    generators::models::run(&conf_def, &opt)?;

    Ok(())
}

/// If the path is the root of a project, add on ./src/models
/// If the use is giving a path directly allow it to fly
fn clean_code_output_path(opt: &mut GenerateOption) {
    if is_project_path(&opt.output_path) {
        let mut new_path = PathBuf::from(&opt.output_path);
        new_path.push("src");
        new_path.push("models");
        opt.output_path = new_path;
    }
}

fn is_project_path(path: &PathBuf) -> bool {
    if !path.exists() || !path.is_dir() {
        return false;
    }
    let mut src = PathBuf::from(path);
    src.push("src");
    if !src.exists() || !src.is_dir() {
        return false;
    }
    let mut cargo_toml = PathBuf::from(path);
    cargo_toml.push("Cargo.toml");
    if !cargo_toml.exists() || !cargo_toml.is_file() {
        return false;
    }
    true
}

/// tests the underlying database connection set with DATABASE_URL
pub async fn test_connection() -> Result<()> {
    let result = welds::connections::connect_from_env().await;
    match result {
        Ok(_) => {
            println!("Database connected successfully");
            std::process::exit(0);
        }
        Err(err) => {
            eprintln!("Not able to connect to database");
            log::debug!("{:?}", err);
            std::process::exit(1);
        }
    }
}
