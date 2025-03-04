use super::{Column, DbProvider, Relation};
use serde::{Deserialize, Serialize};
use welds::detect::{ColumnDef, DataType, TableDef};
use welds::model_traits::TableIdent;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct Table {
    pub schema: Option<String>,    // What schema this table belongs to
    pub name: String,              // Table name
    pub manual_update: bool,       // Tell welds to ignore this Def when scanning the database
    model: Option<String>,         // value Default to singularized version of table name
    pub r#type: String,            // This could be a table or view
    pub columns: Vec<Column>,      // What are the columns on this table
    pub belongs_to: Vec<Relation>, // list of objects this object belongs to
    pub belongs_to_one: Vec<Relation>, // the objects this object belongs to
    pub has_one: Vec<Relation>,    // which object this object has one of
    pub has_many: Vec<Relation>,   // what objects this object has many of
    pub database: DbProvider,      // what DB this object was scanned from.
}

fn type_str(ty: DataType) -> &'static str {
    match ty {
        DataType::View => "view",
        DataType::Table => "table",
    }
}

impl Table {
    pub fn new(table_def: &TableDef, provider: DbProvider) -> Self {
        let mut t = Table {
            manual_update: false,
            name: table_def.ident().name().to_string(),
            schema: table_def.ident().schema().map(|s| s.to_string()),
            model: None,
            columns: vec![],
            r#type: type_str(table_def.ty()).to_string(),
            belongs_to: table_def.belongs_to().iter().map(|x| x.into()).collect(),
            belongs_to: table_def.belongs_to_one().iter().map(|x| x.into()).collect(),
            has_one: table_def.has_one().iter().map(|x| x.into()).collect(),
            has_many: table_def.has_many().iter().map(|x| x.into()).collect(),
            database: provider,
        };
        t.update_cols_from(table_def.columns());
        t
    }

    pub fn update_from(&mut self, table_def: &TableDef, provider: DbProvider) {
        if self.manual_update {
            return;
        }
        self.name = table_def.ident().name().to_string();
        self.schema = table_def.ident().schema().map(|s| s.to_string());
        self.r#type = type_str(table_def.ty()).to_string();
        self.belongs_to = table_def.belongs_to().iter().map(|x| x.into()).collect();
        self.belongs_to_one = table_def.belongs_to_one().iter().map(|x| x.into()).collect();
        self.has_one = table_def.has_one().iter().map(|x| x.into()).collect();
        self.has_many = table_def.has_many().iter().map(|x| x.into()).collect();
        self.update_cols_from(table_def.columns());
        self.database = provider;
    }

    fn update_cols_from(&mut self, cols: &[ColumnDef]) {
        let col_names: Vec<&str> = cols.iter().map(|x| x.name()).collect();
        // Remove Deleted tables
        self.columns
            .retain(|c| col_names.contains(&c.db_name.as_str()));
        // Build a list of new columns to add.
        let mut to_add = Vec::default();
        // Add or update
        for col in cols {
            let existing = self.columns.iter_mut().find(|c| c.db_name == col.name());
            match existing {
                Some(existing) => existing.update_from(col),
                None => to_add.push(Column::new(col)),
            }
        }
        // append new tables
        self.columns.append(&mut to_add);
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
        TableIdent::new(&self.name, self.schema.as_ref())
    }
}
