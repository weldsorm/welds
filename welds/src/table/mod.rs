use crate::errors::Result;
use sqlx::database::HasArguments;
use sqlx::TypeInfo;

pub trait TableInfo {
    /// the unique name (schema + tablename) that identities this database object
    fn identifier() -> &'static str;
}

#[derive(Clone, PartialEq)]
pub struct Column {
    name: String,
    dbtype: String,
}

impl Column {
    pub fn new<DB, T>(name: impl Into<String>) -> Self
    where
        DB: sqlx::Database,
        T: sqlx::Type<DB>,
    {
        Self {
            name: name.into(),
            dbtype: T::type_info().name().to_owned(),
        }
    }

    pub fn name<'a>(&'a self) -> &'a str {
        self.name.as_str()
    }
    pub fn dbtype<'a>(&'a self) -> &'a str {
        self.dbtype.as_str()
    }
}

pub trait TableColumns<DB> {
    // Used to identify models that have N columns in their primary_key
    fn primary_keys() -> Vec<Column>;
    fn columns() -> Vec<Column>;
}

pub trait UniqueIdentifier<DB> {
    /// The column that is used to uniquely identify a row.
    fn id_column() -> Column;
}

pub trait WriteToArgs<DB> {
    fn bind<'args>(
        &self,
        column: &str,
        args: &mut <DB as HasArguments<'args>>::Arguments,
    ) -> Result<()>
    where
        DB: sqlx::Database;
}

pub trait HasSchema {
    type Schema: Default + TableInfo;
}

#[cfg(feature = "detect")]
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct TableIdent {
    pub schema: Option<String>,
    pub name: String,
}
impl TableIdent {
    pub fn equals(&self, schema: &Option<String>, name: &str) -> bool {
        &self.schema == schema && self.name == name
    }
}

#[cfg(feature = "detect")]
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ColumnDef {
    pub name: String,
    pub ty: String,
    pub null: bool,
    pub primary_key: bool,
    pub updatable: bool,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
#[cfg(feature = "detect")]
pub enum DataType {
    Table,
    View,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
#[cfg(feature = "detect")]
pub struct TableDef {
    pub ident: TableIdent,
    pub ty: DataType,
    pub columns: Vec<ColumnDef>, // What are the columns on this table
}
