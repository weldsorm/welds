use super::TableState;
use crate::detect::TableDef;
use crate::errors::{Result, WeldsError};
use crate::table::TableIdent;

pub mod change;
pub mod drop;

pub struct Table(TableDef);

/// Gets all table that can them be altered via migrations
pub fn alter_table(table_state: &TableState, tablename: impl Into<String>) -> Result<Table> {
    let tablename: String = tablename.into();
    let ident = TableIdent::parse(&tablename);

    // Find the table that matches the given ident
    let found = table_state
        .0
        .iter()
        .find(|&t| t.ident() == &ident)
        .ok_or(WeldsError::MissingTable(ident.clone()))?;

    Ok(Table(found.clone()))
}

impl Table {
    pub fn change(self, column_name: impl Into<String>) -> change::Change {
        change::Change::new(self.0, column_name.into())
    }

    pub fn drop(self) -> drop::Drop {
        drop::Drop::new(self.0)
    }
}

#[cfg(feature = "mock")]
/// This module allows you to mock Table
pub mod mock {
    use super::*;
    use crate::detect::table_def::mock::MockTableDef;

    impl Table {
        pub fn mock(t: MockTableDef) -> Table {
            Table(t.build())
        }
    }
}
