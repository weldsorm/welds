use super::TableState;
use crate::detect::TableDef;
use crate::errors::{Result, WeldsError};
use crate::migrations::types::Type;
use crate::model_traits::TableIdent;

pub mod add_column;
pub mod change;
pub mod drop;

pub struct Table(TableDef);

/// Start a migration to change a table.
pub fn change_table(table_state: &TableState, tablename: impl Into<String>) -> Result<Table> {
    let tablename: String = tablename.into();

    // The table state include the schema for public/dto.
    // make sure is it on the TableIdent if needed
    let syntax = table_state.0.first().map(|t| t.syntax());
    let default_namespace = syntax
        .and_then(TableIdent::default_namespace)
        .map(|s| s.to_string());
    let mut ident = TableIdent::parse(&tablename);
    ident.schema = ident.schema.or(default_namespace);

    // If no schema was given, and we still don't know the default schema. look at all the table

    // Find the tables that matches the given tablename
    let mut search: Vec<&TableDef> = table_state
        .0
        .iter()
        .filter(|&t| t.ident().name() == ident.name())
        .collect();

    // If we know the schema, filter on that as well.
    if ident.schema().is_some() {
        search = search
            .drain(..)
            .filter(|&t| t.ident.schema() == ident.schema())
            .collect()
    }

    // Make sure we found exactly ONE table. no more, no less
    if search.is_empty() {
        Err(WeldsError::MissingTable(ident.clone()))?;
    }
    if search.len() > 1 {
        let err = format!("The table {} is ambiguous. This table was found under multiple schemanames. Please include the schema name when migrating.",tablename);
        Err(WeldsError::MigrationError(err))?;
    }
    let found: &TableDef = search.pop().unwrap();

    Ok(Table(found.clone()))
}

impl Table {
    /// Alter a column on this table
    pub fn change(self, column_name: impl Into<String>) -> change::Change {
        change::Change::new(self.0, column_name.into())
    }

    /// Drop this table from the database
    pub fn drop(self) -> drop::Drop {
        drop::Drop::new(self.0)
    }

    /// Add a new column onto this table
    pub fn add_column(self, column_name: impl Into<String>, ty: Type) -> add_column::AddColumn {
        add_column::AddColumn::new(self.0, column_name.into(), ty)
    }
}

/// This module allows you to mock Table
#[cfg(feature = "mock")]
pub mod mock {
    use super::*;
    use crate::detect::table_def::mock::MockTableDef;

    impl Table {
        pub fn mock(t: MockTableDef) -> Table {
            Table(t.build())
        }
    }
}

#[cfg(test)]
mod tests;
