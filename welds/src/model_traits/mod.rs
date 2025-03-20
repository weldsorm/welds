/// ***********************************************************************************
/// These are all the trait and struct used to connect a rust Struct to a database driver
/// ***********************************************************************************
pub mod hooks;

#[cfg(test)]
mod tests;

/// tells welds what tablename and schema name should used to get data for an Entity
/// This does on the Schema Object NOT the model
pub trait TableInfo {
    /// the unique name (schema + tablename) that identities this database object
    fn identifier() -> &'static [&'static str];
}

/// The db column name to use for a field
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Column {
    name: String,
    rust_type: String,
    nullable: bool,
}

impl Column {
    pub fn new(name: impl Into<String>, rust_type: impl Into<String>, nullable: bool) -> Self {
        let rust_type = rust_type.into();
        let rust_type: String = rust_type.chars().filter(|c| !c.is_whitespace()).collect();
        Self {
            name: name.into(),
            rust_type,
            nullable,
        }
    }
    /// The name of the column in the database
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    /// if the underlying database column could return a null value
    /// used to know if you can query for None/Some
    pub fn nullable(&self) -> bool {
        self.nullable
    }
    /// The name of the column in the database
    pub fn rust_type(&self) -> &str {
        self.rust_type.as_str()
    }
}

/// How welds knows what columns exist on your model
/// This trait is impl by the model's schema not the model
pub trait TableColumns {
    type ColumnStruct: TableColumns + Default;

    // Used to identify models that have N columns in their primary_key
    fn primary_keys() -> Vec<Column>;
    fn columns() -> Vec<Column>;
}

/// If the model can be uniquely identifed by a single column,
/// This is used to create get_by_id methods
pub trait UniqueIdentifier {
    /// The column that is used to uniquely identify a row.
    fn id_column() -> Column;
}

use crate::errors::Result;
use crate::query::clause::ParamArgs;

pub trait WriteToArgs {
    fn bind<'s, 'c, 'a, 'p>(&'s self, column: &'c str, args: &'a mut ParamArgs<'p>) -> Result<()>
    where
        's: 'p;
}

pub trait ColumnDefaultCheck {
    fn col_is_default(&self, column: &str) -> Result<bool>;
}

/// Update self from the contents of a row
pub trait UpdateFromRow {
    fn update_from_row(&mut self, row: &mut crate::Row) -> crate::errors::Result<()>;
}

/// Used to link a models schema to the model
pub trait HasSchema: Sync + Send {
    type Schema: Default + TableInfo;
}

/// Returns the Value of the PK of a model
pub trait PrimaryKeyValue {
    type PrimaryKeyType;
    /// Returns the value of a model's primary key(s)
    fn primary_key_value(&self) -> Self::PrimaryKeyType;
}

/// Used to check if a Foreign Key is equal to a value
pub trait ForeignKeyPartialEq<Rhs> {
    /// return true if the Foreign Key value equals the passed in value
    /// false if the values don't match OR object doesn't have the column
    fn eq(&self, foreign_key_column: &str, other: &Rhs) -> bool;
}

mod tableident;
pub use tableident::TableIdent;
