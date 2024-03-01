use super::HasSchema;
use super::TableColumns;
use super::TableInfo;
use crate::Syntax;

/// a unique identifier for a table.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct TableIdent {
    pub(crate) schema: Option<String>,
    pub(crate) name: String,
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
    pub fn from_model<T>() -> TableIdent
    where
        T: HasSchema,
        <T as HasSchema>::Schema: TableInfo + TableColumns,
    {
        let fullname = <T as HasSchema>::Schema::identifier().join(".");
        Self::parse(&fullname)
    }

    /// Returns the name of the table. Table only on schema_name
    pub fn new(table_name: impl Into<String>, schema_name: Option<impl Into<String>>) -> Self {
        let table_name = table_name.into();
        let schema_name = schema_name.map(|x| x.into());
        Self {
            name: table_name,
            schema: schema_name,
        }
    }

    /// Returns the name of the table. Table only on schema_name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the schema_name part of the table identifier.
    /// If schema_name is not given, It will be None
    pub fn schema(&self) -> Option<&str> {
        self.schema.as_deref()
    }

    /// Parse a string into a TableIdent
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

    /// returns True if a schema_name/table_name match this TableIdent
    pub fn equals(&self, schema: &Option<String>, name: &str) -> bool {
        &self.schema == schema && self.name == name
    }

    /// returns the default namespace that is used for a syntax
    pub fn default_namespace(syntax: Syntax) -> Option<&'static str> {
        match syntax {
            Syntax::Mssql => Some("dbo"),
            Syntax::Postgres => Some("public"),
            // NOTE if schema is left out, the mysql query uses the name of the db in the connection
            Syntax::Mysql => None,
            Syntax::Sqlite => None,
        }
    }
}
