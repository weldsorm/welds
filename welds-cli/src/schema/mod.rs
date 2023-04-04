use crate::adapters::TableIdent;
use crate::errors::{Result, WeldsError};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub tables: Vec<Table>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Table {
    pub name: String,         // Table name
    model: Option<String>,    // value Default to singularized version of table name
    pub r#type: String,       // This could be a table or view
    pub schema: Schema,       // What schema this table belongs to
    pub columns: Vec<Column>, // What are the columns on this table
    #[serde(default = "all_abilities")]
    pub abilities: Vec<Ability>,
}

impl Table {
    pub fn new(name: String, schema: String, r#type: Option<String>) -> Self {
        Table {
            name,
            schema: Schema { name: schema },
            model: None,
            columns: vec![],
            r#type: r#type.unwrap_or_else(|| "table".to_string()),
            abilities: vec![],
        }
    }

    /// Returns the name of the module this table will be placed in
    pub fn module_name(&self) -> String {
        use inflector::Inflector;
        let start = match &self.model {
            Some(s) => s.to_string(),
            None => self.name.to_singular(),
        };
        start.to_snake_case()
    }

    /// Returns the name of the struct this table will generate
    pub fn struct_name(&self) -> String {
        use inflector::Inflector;
        let start = match &self.model {
            Some(s) => s.to_string(),
            None => self.name.to_singular(),
        };
        start.to_class_case()
    }

    /// return how this table is identified by the database
    pub(crate) fn ident(&self) -> TableIdent {
        TableIdent {
            name: self.name.clone(),
            schema: self.schema.name.clone(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Schema {
    pub name: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Column {
    pub name: String,
    pub r#type: String,
    #[serde(default)]
    pub null: bool,
}

fn all_abilities() -> Vec<Ability> {
    vec![
        Ability::Create,
        Ability::Update,
        Ability::Select,
        Ability::Delete,
    ]
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum Ability {
    Create,
    Update,
    Delete,
    Select,
}
