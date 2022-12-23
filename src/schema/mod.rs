use crate::errors::{Result, WeldsError};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Reads in a schema config file and parses it into a config
pub(crate) fn read(path: &PathBuf) -> Result<Config> {
    let yaml_str =
        std::fs::read_to_string(path).map_err(|_| WeldsError::ReadError(path.clone()))?;
    let config: std::result::Result<Config, serde_yaml::Error> = serde_yaml::from_str(&yaml_str);
    match config {
        Err(err) => Err(WeldsError::ConfigReadError((path.clone(), err))),
        Ok(config) => Ok(config),
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Config {
    pub tables: Vec<Table>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Table {
    pub name: String,          // Table name
    pub model: Option<String>, // value Default to singularized version of table name
    pub schema: Vec<Schema>,
    #[serde(default = "all_abilities")]
    pub abilities: Vec<Ability>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Schema {
    pub name: String,
    pub r#type: String,
}

fn all_abilities() -> Vec<Ability> {
    vec![
        Ability::Create,
        Ability::Update,
        Ability::Select,
        Ability::Delete,
    ]
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) enum Ability {
    Create,
    Update,
    Delete,
    Select,
}
