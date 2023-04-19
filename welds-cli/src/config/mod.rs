use crate::errors::{Result, WeldsError};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

mod table;
pub use table::Table;

mod column;
pub use column::Column;

///
/// This module holds the Struct and functions for welds.yaml configs
///

/// Reads in a schema config file and parses it into a config
pub(crate) fn read(path: &PathBuf) -> Result<Config> {
    let yaml_str =
        std::fs::read_to_string(path).map_err(|_| WeldsError::ReadError(path.clone()))?;
    let config: std::result::Result<Config, serde_yaml::Error> = serde_yaml::from_str(&yaml_str);
    match config {
        Err(_err) => Err(WeldsError::ConfigReadError(path.clone())),
        Ok(config) => Ok(config),
    }
}

/// Writes a schema config file to disk
pub(crate) fn write(path: &PathBuf, config: &Config) -> Result<()> {
    let yaml = serde_yaml::to_string(config).map_err(|_| WeldsError::ConfigWrite)?;
    std::fs::write(path, yaml.as_bytes())?;
    Ok(())
}

#[derive(Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub tables: Vec<Table>,
}

impl Config {
    pub(crate) fn remove_missing(&mut self, tables: &[welds::table::TableDef]) {
        let idents: Vec<_> = tables.iter().map(|x| x.ident()).collect();
        // Remove Deleted tables
        self.tables
            .retain(|x| x.manual_update || idents.contains(&&x.ident()));
    }

    pub(crate) fn add_update(&mut self, tables: &[welds::table::TableDef], provider: DbProvider) {
        // Build a list of new columns to add.
        let mut to_add = Vec::default();
        // Add or update
        for t in tables {
            let existing = self.tables.iter_mut().find(|x| &x.ident() == t.ident());
            match existing {
                Some(existing) => existing.update_from(t, provider),
                None => to_add.push(Table::new(t, provider)),
            }
        }
        // append new tables
        self.tables.append(&mut to_add);
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Relation {
    pub schema: Option<String>, // What schema this table belongs to
    pub tablename: String,      // Table name
    pub foreign_key: String,    // The foreign_key column
}

impl From<&welds::table::RelationDef> for Relation {
    fn from(value: &welds::table::RelationDef) -> Self {
        Relation {
            schema: value.other_table.schema.clone(),
            tablename: value.other_table.name.to_owned(),
            foreign_key: value.foreign_key.to_owned(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum DbProvider {
    Postgres,
    Mysql,
    Mssql,
    Sqlite,
}
