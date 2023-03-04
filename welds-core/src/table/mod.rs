use sqlx::TypeInfo;
pub(crate) mod col_writer;

pub trait TableInfo {
    /// the unique name (schema + tablename) that identities this database object
    fn identifier() -> &'static str;
}

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
    fn columns() -> Vec<Column>;
}
