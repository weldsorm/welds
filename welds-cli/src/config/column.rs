use serde::{Deserialize, Serialize};
use welds::table::ColumnDef;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Column {
    pub db_name: String,
    pub db_type: String,
    pub model_name: String,
    #[serde(default)]
    pub is_null: bool,
    #[serde(default)]
    pub primary_key: bool,
    #[serde(default)]
    pub writeable: bool,
}

impl Column {
    pub fn new(def: &ColumnDef) -> Self {
        use inflector::Inflector;
        Column {
            db_name: def.name.to_owned(),
            db_type: def.ty.to_owned(),
            model_name: def.name.to_snake_case().to_owned(),
            is_null: def.null,
            primary_key: def.primary_key,
            writeable: def.updatable,
        }
    }

    pub fn update_from(&mut self, def: &ColumnDef) {
        self.db_name = def.name.to_owned();
        self.db_type = def.ty.to_owned();
        self.is_null = def.null;
        self.primary_key = def.primary_key;
        self.writeable = def.updatable;
    }
}
