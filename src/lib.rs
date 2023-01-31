pub mod adapters;
pub mod commands;
pub(crate) mod database;
mod errors;
pub mod generators;
pub mod schema;
use adapters::TableIdent;
use errors::Result;
pub use errors::WeldsError;
use schema::Table;
use std::path::PathBuf;

pub async fn update(schema_path: PathBuf, identifier: Option<String>) -> Result<()> {
    let conn = crate::database::connect().await?;
    let identifier = identifier.as_ref().map(|x| TableIdent::new(x, &conn));
    let mut tables = crate::adapters::schema(&conn).await?;
    let tables: Vec<Table> = tables
        .drain(..)
        .filter(|t| !IGNORE_TABLES.contains(&t.name.as_str()))
        .collect();

    let mut config = schema::read(&schema_path).unwrap_or_default();

    match identifier {
        Some(identifier) => {
            let table = tables.iter().find(|t| t.ident() == identifier);
            config.tables = update_single(&identifier, table, &config.tables);
        }
        None => config.tables = tables,
    };
    schema::write(&schema_path, &config)?;
    Ok(())
}

fn update_single(ident: &TableIdent, table: Option<&Table>, current: &[Table]) -> Vec<Table> {
    let mut found = false;
    let mut new_list: Vec<Table> = current
        .iter()
        .filter_map(|t| {
            if &t.ident() != ident {
                return Some(t.clone());
            }
            found = true;
            table.map(|nt| nt.clone())
        })
        .collect();
    // When the table is new to the schema Definition
    if let Some(table) = table {
        if !found {
            new_list.push(table.clone());
        }
    }

    new_list
}

const IGNORE_TABLES: &'static [&'static str] =
    &["_sqlx_migrations", "sqlite_schema", "sqlite_temp_schema"];

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
