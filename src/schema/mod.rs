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
pub struct Config {
    pub tables: Vec<Table>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Table {
    pub name: String,      // Table name
    model: Option<String>, // value Default to singularized version of table name
    pub schema: Vec<Schema>,
    #[serde(default = "all_abilities")]
    pub abilities: Vec<Ability>,
}

impl Table {
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
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Schema {
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Ability {
    Create,
    Update,
    Delete,
    Select,
}
