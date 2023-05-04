use anyhow::Result;
use sqlx::database::HasArguments;
use sqlx::TypeInfo;

pub trait TableInfo {
    /// the unique name (schema + tablename) that identities this database object
    fn identifier() -> &'static [&'static str];
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Column {
    name: String,
    dbtype: String,
    nullable: bool,
}

impl Column {
    pub fn new<DB, T>(name: impl Into<String>, nullable: bool) -> Self
    where
        DB: sqlx::Database,
        T: sqlx::Type<DB>,
    {
        Self {
            name: name.into(),
            dbtype: T::type_info().name().to_owned(),
            nullable,
        }
    }
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    pub fn dbtype(&self) -> &str {
        self.dbtype.as_str()
    }
    pub fn nullable(&self) -> bool {
        self.nullable
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
    fn bind(&self, column: &str, args: &mut <DB as HasArguments<'_>>::Arguments) -> Result<()>
    where
        DB: sqlx::Database;
}

pub trait WriteBulkArrayToArgs<DB> {
    fn bind(
        data: &[&Self],
        column: &Column,
        args: &mut <DB as HasArguments<'_>>::Arguments,
    ) -> Result<()>
    where
        DB: sqlx::Database;
}

pub trait HasSchema {
    type Schema: Default + TableInfo;
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct TableIdent {
    pub schema: Option<String>,
    pub name: String,
}

impl std::fmt::Display for TableIdent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(s) = &self.schema {
            f.write_str(s)?;
            f.write_str(".")?;
        }
        f.write_str(&self.name)?;
        Ok(())
    }
}

impl TableIdent {
    pub fn parse(raw: &str) -> Self {
        let parts: Vec<&str> = raw.split('.').collect();
        let parts: Vec<&str> = parts.iter().rev().take(2).cloned().collect();
        let name = parts
            .first()
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
