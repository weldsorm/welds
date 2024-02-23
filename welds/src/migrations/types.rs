#[derive(Debug, Clone, PartialEq, Eq)]
/// These are types that are defined in migrations.
/// They will get translated into DB types
pub enum Type {
    Bool,
    IntSmall,
    Int,
    IntBig,
    String(u32),
    Text,
    Float,
    FloatBig,
    Blob,
    Raw(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// These are types that are defined in migrations for Ids.
/// They will get translated into DB types
pub enum IdType {
    IntSmall,
    Int,
    IntBig,
    String(u32),
    Text,
    Uuid,
    Raw(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// These are types of Indexes that can be created
pub enum Index {
    Default,
    Unique,
}

pub trait ToDbType<DB: sqlx::Database> {
    fn dbtype(&self) -> String;
}

#[cfg(feature = "sqlite")]
impl ToDbType<sqlx::Sqlite> for Type {
    fn dbtype(&self) -> String {
        use Type::*;
        match self {
            Bool => "BOOLEAN".to_owned(),
            IntSmall => "INT".to_owned(),
            Int => "INT".to_owned(),
            IntBig => "INT".to_owned(),
            String(size) => format!("VARCHAR({})", size),
            Text => "TEXT".to_owned(),
            Float => "FLOAT".to_owned(),
            FloatBig => "FLOAT".to_owned(),
            Blob => "BLOB".to_owned(),
            Raw(r) => r.clone(),
        }
    }
}

#[cfg(feature = "sqlite")]
impl ToDbType<sqlx::Sqlite> for IdType {
    fn dbtype(&self) -> String {
        use IdType::*;
        match self {
            IntSmall => "INTEGER".to_owned(),
            Int => "INTEGER".to_owned(),
            IntBig => "INTEGER".to_owned(),
            String(size) => format!("VARCHAR({})", size),
            Text => "TEXT".to_owned(),
            Uuid => "VARCHAR(36)".to_owned(),
            Raw(r) => r.clone(),
        }
    }
}

#[cfg(feature = "postgres")]
impl ToDbType<sqlx::Postgres> for Type {
    fn dbtype(&self) -> String {
        use Type::*;
        match self {
            Bool => "XXXX".to_owned(),
            IntSmall => "XXXX".to_owned(),
            Int => "XXXX".to_owned(),
            IntBig => "XXXX".to_owned(),
            String(size) => format!("XXXX({})", size),
            Text => "XXXX".to_owned(),
            Float => "XXXX".to_owned(),
            FloatBig => "XXXX".to_owned(),
            Blob => "XXXX".to_owned(),
            Raw(r) => r.clone(),
        }
    }
}

#[cfg(feature = "postgres")]
impl ToDbType<sqlx::Postgres> for IdType {
    fn dbtype(&self) -> String {
        use IdType::*;
        match self {
            IntSmall => "YYY".to_owned(),
            Int => "YYY".to_owned(),
            IntBig => "YYY".to_owned(),
            String(size) => format!("YYY({})", size),
            Text => "YYY".to_owned(),
            Uuid => "YYY(36)".to_owned(),
            Raw(r) => r.clone(),
        }
    }
}
