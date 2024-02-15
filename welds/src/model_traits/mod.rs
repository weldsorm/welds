/// ***********************************************************************************
/// These are all the trait and struct used to connect a rust Struct to a database driver
/// ***********************************************************************************

/// tells welds what tablename and schema name should used to get data for an Entity
/// This does on the Schema Object NOT the model
pub trait TableInfo {
    /// the unique name (schema + tablename) that identities this database object
    fn identifier() -> &'static [&'static str];
}

/// The db column name to use for a field
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Column {
    name: String,
    nullable: bool,
}

#[cfg(test)]
impl Column {
    pub fn mock(name: impl Into<String>, nullable: bool) -> Column {
        Column {
            name: name.into(),
            nullable,
        }
    }
}

impl Column {
    pub fn new<T>(name: impl Into<String>, nullable: bool) -> Self {
        Self {
            name: name.into(),
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
}

/// How welds knows what columns exist on your model
/// This trait is impl by the model's schema not the model
pub trait TableColumns {
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
    fn bind<'s, 'c, 'a>(&'s self, column: &'c str, args: &'s mut ParamArgs<'a>) -> Result<()>;
}

//pub trait WriteToArgs<DB> {
//    fn bind(&self, column: &str, args: &mut <DB as HasArguments<'_>>::Arguments) -> Result<()>
//    where
//        DB: sqlx::Database;
//}

//pub trait WriteBulkArrayToArgs<DB> {
//    fn bind(
//        data: &[&Self],
//        column: &Column,
//        args: &mut <DB as HasArguments<'_>>::Arguments,
//    ) -> Result<()>
//    where
//        DB: sqlx::Database;
//}

/// Used to link a models schema to the model
pub trait HasSchema: Sync + Send {
    type Schema: Default + TableInfo;
}

/// a unique identifier for a table.
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