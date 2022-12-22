use serde::{Deserialize, Serialize};

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
