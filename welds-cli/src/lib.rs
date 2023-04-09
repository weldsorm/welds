pub mod commands;
pub mod errors;
pub mod generators;
pub mod schema;
use crate::errors::{Result, WeldsError};
use schema::DbProvider;
use schema::Table;
use std::path::PathBuf;
use welds::table::TableIdent;

pub async fn update(schema_path: PathBuf, identifier: Option<String>) -> Result<()> {
    use welds::database::Pool;
    use welds::detect::find_tables;
    let identifier = identifier.as_ref().map(|x| TableIdent::parse(x));
    let unknown_pool = welds::database::connect().await?;

    let provider = match unknown_pool {
        Pool::Postgres(_) => DbProvider::Postgres,
        Pool::MySql(_) => DbProvider::Mysql,
        Pool::Mssql(_) => DbProvider::Mssql,
        Pool::Sqlite(_) => DbProvider::Sqlite,
    };

    let mut tables = match unknown_pool {
        Pool::Postgres(conn) => find_tables(&conn).await?,
        Pool::Mssql(conn) => find_tables(&conn).await?,
        Pool::MySql(conn) => find_tables(&conn).await?,
        Pool::Sqlite(conn) => find_tables(&conn).await?,
    };

    if let Some(ident) = identifier {
        tables = tables.drain(..).filter(|t| t.ident() == &ident).collect()
    }

    let mut config = schema::read(&schema_path).unwrap_or_default();

    //match identifier {
    //    Some(identifier) => {
    //        let table = tables.iter().find(|t| t.ident() == &identifier);
    //        config.tables = update_single(&identifier, table, &config.tables);
    //    }
    //    None => config.tables = tables,
    //};

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

#[derive(Debug, Default)]
pub struct GenerateOption {
    pub schema_path: PathBuf,
    pub output_path: PathBuf,
    pub table: Option<String>,
    pub force: bool,
}

pub fn generate(mut opt: GenerateOption) -> Result<()> {
    if !opt.schema_path.exists() {
        return Err(WeldsError::MissingSchemaFile(opt.schema_path));
    }

    let config = schema::read(&opt.schema_path)?;

    clean_code_output_path(&mut opt);
    generators::models::run(&config, &opt)?;

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
