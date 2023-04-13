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

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
#[cfg(feature = "detect")]
pub struct TableIdent {
    pub schema: Option<String>,
    pub name: String,
}

#[cfg(feature = "detect")]
impl TableIdent {
    pub fn parse(raw: &str) -> Self {
        let parts: Vec<&str> = raw.split(".").collect();
        let parts: Vec<&str> = parts.iter().rev().take(2).cloned().collect();
        let name = parts
            .get(0)
            .cloned()
            .map(|x| x.to_owned())
            .unwrap_or_default();
        let schema = parts.get(1).cloned().map(|x| x.to_owned());
        Self { schema, name }
    }
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

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg(feature = "detect")]
pub enum DataType {
    Table,
    View,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
#[cfg(feature = "detect")]
pub struct RelationDef {
    /// The Other side of this relationship
    pub other_table: TableIdent,
    /// this is the foreign_key side column regardless of which side this defines
    pub foreign_key: String,
    /// this is the column the fk point to, regardless of which side this defines
    pub primary_key: String,
}

#[cfg(feature = "detect")]
impl RelationDef {
    pub(crate) fn new(ident: TableIdent, foreign_key: &str, primary_key: &str) -> Self {
        Self {
            other_table: ident,
            foreign_key: foreign_key.to_owned(),
            primary_key: primary_key.to_owned(),
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
#[cfg(feature = "detect")]
pub struct TableDef {
    pub(crate) ident: TableIdent,
    pub(crate) ty: DataType,
    pub(crate) columns: Vec<ColumnDef>, // What are the columns on this table
    pub(crate) has_many: Vec<RelationDef>,
    pub(crate) belongs_to: Vec<RelationDef>,
}

#[cfg(feature = "detect")]
impl TableDef {
    pub fn ident<'a>(&'a self) -> &'a TableIdent {
        &self.ident
    }
    pub fn ty(&self) -> DataType {
        self.ty
    }
    pub fn columns<'a>(&'a self) -> &'a [ColumnDef] {
        &self.columns
    }
    pub fn has_many<'a>(&'a self) -> &'a [RelationDef] {
        &self.has_many
    }
    pub fn belongs_to<'a>(&'a self) -> &'a [RelationDef] {
        &self.belongs_to
    }
}
