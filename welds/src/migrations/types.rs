use crate::Syntax;
use crate::writers::types::Pair;
use crate::writers::types::get_pairs;
use std::iter::Iterator;

#[derive(Debug, Clone, PartialEq, Eq)]
/// These are types that are defined in migrations.
/// They will get translated into DB types
pub enum Type {
    Bool,
    IntSmall,
    Int,
    IntBig,
    String,
    StringSized(u32),
    Text,
    Json,
    Float,
    FloatBig,
    Binary,
    BinarySized(u32),
    Uuid,
    Date,
    Time,
    Datetime,
    /// Datetime + (offset or timezone)
    DatetimeZone,
    Raw(String),
}

impl Type {
    // Returns the rust type to use for a given migration type
    pub fn rust_type(&self) -> Option<String> {
        Some(match self {
            Type::Bool => "bool".to_owned(),
            Type::IntSmall => "i16".to_owned(),
            Type::Int => "i32".to_owned(),
            Type::IntBig => "i64".to_owned(),
            Type::String => "String".to_owned(),
            Type::StringSized(_) => "String".to_owned(),
            Type::Text => "String".to_owned(),
            Type::Json => "serde_json::Value".to_owned(),
            Type::Float => "f32".to_owned(),
            Type::FloatBig => "f64".to_owned(),
            Type::Binary => "Vec<u8>".to_owned(),
            Type::BinarySized(_) => "Vec<u8>".to_owned(),
            Type::Uuid => "Uuid".to_owned(),
            Type::Date => "chrono::NaiveDate".to_owned(),
            Type::Time => "chrono::NaiveTime".to_owned(),
            Type::Datetime => "chrono::NaiveDateTime".to_owned(),
            Type::DatetimeZone => "chrono::DateTime<chrono::Utc>".to_owned(),
            Type::Raw(_) => return None,
        })
    }

    /// Returns the DB type to use in a migration
    pub fn db_type(&self, syntax: Syntax) -> String {
        if let Type::BinarySized(size) = self {
            return match syntax {
                Syntax::Mysql | Syntax::Mssql => format!("VARBINARY({})", size),
                Syntax::Postgres => "BYTEA".to_owned(),
                Syntax::Sqlite => "BLOB".to_owned(),
            };
        }

        let pairs = get_pairs(syntax);
        find_db_type(self, &pairs[..], false)
    }

    /// Returns the DB type to use in a migration for Id columns
    pub fn db_id_type(&self, syntax: Syntax) -> String {
        if let Type::BinarySized(size) = self {
            return match syntax {
                Syntax::Mysql | Syntax::Mssql => format!("VARBINARY({})", size),
                Syntax::Postgres => "BYTEA".to_owned(),
                Syntax::Sqlite => "BLOB".to_owned(),
            };
        }

        let pairs = get_pairs(syntax);
        find_db_type(self, &pairs[..], true)
    }

    /// Reads in a DB type and returns a type version of it.
    pub(crate) fn parse_db_type(syntax: Syntax, db_type: &str) -> Type {
        let db_type = db_type.trim().replace(' ', "").to_uppercase();
        let normalized = db_type.as_str();

        if let Some(size) = normalized
            .strip_prefix("VARCHAR(")
            .and_then(|s| s.strip_suffix(')'))
            .filter(|s| !s.eq_ignore_ascii_case("MAX"))
            .and_then(|s| s.parse::<u32>().ok())
        {
            return Type::StringSized(size);
        }
        if let Some(size) = normalized
            .strip_prefix("NVARCHAR(")
            .and_then(|s| s.strip_suffix(')'))
            .filter(|s| !s.eq_ignore_ascii_case("MAX"))
            .and_then(|s| s.parse::<u32>().ok())
        {
            return Type::StringSized(size);
        }
        if let Some(size) = normalized
            .strip_prefix("CHAR(")
            .and_then(|s| s.strip_suffix(')'))
            .filter(|s| !s.eq_ignore_ascii_case("MAX"))
            .and_then(|s| s.parse::<u32>().ok())
        {
            return Type::StringSized(size);
        }
        if let Some(size) = normalized
            .strip_prefix("BINARY(")
            .and_then(|s| s.strip_suffix(')'))
            .filter(|s| !s.eq_ignore_ascii_case("MAX"))
            .and_then(|s| s.parse::<u32>().ok())
        {
            return Type::BinarySized(size);
        }
        if let Some(size) = normalized
            .strip_prefix("VARBINARY(")
            .and_then(|s| s.strip_suffix(')'))
            .filter(|s| !s.eq_ignore_ascii_case("MAX"))
            .and_then(|s| s.parse::<u32>().ok())
        {
            return Type::BinarySized(size);
        }

        let pairs = get_pairs(syntax);
        let found = pairs.iter().find(|&p| p.db_type() == db_type);
        let size = found.and_then(|f| f.default_size());

        match size {
            Some(s) => Type::Raw(format!("{}({})", db_type, s)),
            None => Type::Raw(db_type),
        }
    }
}

/// Returns the DB type to use in a migration
fn find_db_type(ty: &Type, pairs: &[Pair], include_id_only: bool) -> String {
    // If the developer want to give us a type use it
    if let Type::Raw(raw) = ty {
        return raw.to_owned();
    }

    let rust_type = ty.rust_type().unwrap();
    let pair = match ty {
        Type::StringSized(_) => pairs
            .iter()
            .filter(|p| include_id_only || !p.id_only())
            .filter(|p| p.is_rust_type(&rust_type))
            .find(|p| p.db_sized())
            .or_else(|| {
                pairs
                    .iter()
                    .filter(|p| include_id_only || !p.id_only())
                    .find(|p| p.is_rust_type(&rust_type))
            })
            .unwrap(),
        _ => pairs
            .iter()
            .filter(|p| include_id_only || !p.id_only())
            .find(|p| p.is_rust_type(&rust_type))
            .unwrap(),
    };

    // If the pair is sized. Make sure we add the size info
    if pair.db_sized() {
        let size = match ty {
            Type::StringSized(s) => s.to_string(),
            Type::BinarySized(s) => s.to_string(),
            _ => pair.default_size().unwrap().to_owned(),
        };
        return format!("{}({})", pair.db_type(), size);
    }

    pair.db_type().to_string()
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// These are types of Indexes that can be created
pub enum Index {
    Default,
    Unique,
    ForeignKey((String, String, OnDelete)),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// These are types of Indexes that can be created
pub enum OnDelete {
    /// Automatically deletes rows in the child table that reference the deleted row in the parent table.
    Cascade,
    /// Sets the foreign key column in the child table to NULL when the referenced row in the parent table is deleted.
    SetNull,
    /// Sets the foreign key column in the child table to its default value when the referenced row in the parent table is deleted.
    /// WARNING: Requires the foreign key column to have a default value defined.
    SetDefault,
    /// Prevents the deletion of the referenced row in the parent table if any child rows reference it.
    /// This is the default behavior in most databases when ON DELETE is not explicitly specified.
    Restrict,
    /// Similar to RESTRICT, but the check is deferred until the end of the current transaction in some databases.
    NoAction,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn type_should_be_an_int() {
        let db_type = |s| Type::Int.db_type(s);
        assert_eq!(&db_type(Syntax::Sqlite), "INTEGER");
        assert_eq!(&db_type(Syntax::Mysql), "INT");
        assert_eq!(&db_type(Syntax::Postgres), "INT");
        assert_eq!(&db_type(Syntax::Mssql), "INT");
    }

    #[test]
    fn type_should_be_an_int_pk() {
        let db_type = |s| Type::Int.db_id_type(s);
        assert_eq!(&db_type(Syntax::Sqlite), "INTEGER");
        assert_eq!(&db_type(Syntax::Mysql), "INT");
        assert_eq!(&db_type(Syntax::Postgres), "SERIAL");
        assert_eq!(&db_type(Syntax::Mssql), "INT");
    }

    #[test]
    fn type_should_be_a_db_string() {
        let db_type = |s| Type::String.db_type(s);
        assert_eq!(&db_type(Syntax::Sqlite), "TEXT");
        assert_eq!(&db_type(Syntax::Mysql), "VARCHAR(255)");
        assert_eq!(&db_type(Syntax::Postgres), "TEXT");
        assert_eq!(&db_type(Syntax::Mssql), "NVARCHAR(MAX)");
    }

    #[test]
    fn type_should_be_a_small_int() {
        let db_type = |s| Type::IntSmall.db_type(s);
        assert_eq!(&db_type(Syntax::Sqlite), "INTEGER");
        assert_eq!(&db_type(Syntax::Mysql), "SMALLINT");
        assert_eq!(&db_type(Syntax::Postgres), "INT2");
        assert_eq!(&db_type(Syntax::Mssql), "SMALLINT");
    }

    #[test]
    fn type_should_support_varbinary() {
        let db_type = |s| Type::BinarySized(256).db_type(s);
        assert_eq!(&db_type(Syntax::Sqlite), "BLOB");
        assert_eq!(&db_type(Syntax::Mysql), "VARBINARY(256)");
        assert_eq!(&db_type(Syntax::Postgres), "BYTEA");
        assert_eq!(&db_type(Syntax::Mssql), "VARBINARY(256)");
    }

    #[test]
    fn parse_db_type_should_preserve_fixed_length_string_and_binary_types() {
        let varchar_ty = Type::parse_db_type(Syntax::Mysql, "VARCHAR(255)");
        assert_eq!(varchar_ty, Type::StringSized(255));
        assert_eq!(Type::StringSized(255).db_type(Syntax::Mysql), "VARCHAR(255)");
        assert_eq!(Type::StringSized(255).db_type(Syntax::Mssql), "NVARCHAR(255)");
        assert_eq!(Type::StringSized(255).db_type(Syntax::Postgres), "VARCHAR(255)");
        assert_eq!(Type::StringSized(255).db_type(Syntax::Sqlite), "TEXT");

        let char_ty = Type::parse_db_type(Syntax::Mysql, "CHAR(32)");
        assert_eq!(char_ty, Type::StringSized(32));
        assert_eq!(Type::StringSized(32).db_type(Syntax::Mysql), "VARCHAR(32)");
        assert_eq!(Type::StringSized(32).db_type(Syntax::Mssql), "NVARCHAR(32)");
        assert_eq!(Type::StringSized(32).db_type(Syntax::Postgres), "VARCHAR(32)");
        assert_eq!(Type::StringSized(32).db_type(Syntax::Sqlite), "TEXT");

        let nvarchar_ty = Type::parse_db_type(Syntax::Mssql, "NVARCHAR(128)");
        assert_eq!(nvarchar_ty, Type::StringSized(128));
        assert_eq!(Type::StringSized(128).db_type(Syntax::Mysql), "VARCHAR(128)");
        assert_eq!(Type::StringSized(128).db_type(Syntax::Mssql), "NVARCHAR(128)");
        assert_eq!(Type::StringSized(128).db_type(Syntax::Postgres), "VARCHAR(128)");
        assert_eq!(Type::StringSized(128).db_type(Syntax::Sqlite), "TEXT");

        let varbinary_ty = Type::parse_db_type(Syntax::Mysql, "VARBINARY(256)");
        assert_eq!(varbinary_ty, Type::BinarySized(256));
        assert_eq!(Type::BinarySized(256).db_type(Syntax::Mysql), "VARBINARY(256)");
        assert_eq!(Type::BinarySized(256).db_type(Syntax::Mssql), "VARBINARY(256)");
        assert_eq!(Type::BinarySized(256).db_type(Syntax::Postgres), "BYTEA");
        assert_eq!(Type::BinarySized(256).db_type(Syntax::Sqlite), "BLOB");
    }

    #[test]
    fn no_db_type_should_panic() {
        let syntaxes = [
            Syntax::Sqlite,
            Syntax::Mssql,
            Syntax::Mysql,
            Syntax::Postgres,
        ];
        let types = [
            Type::Bool,
            Type::IntSmall,
            Type::Int,
            Type::IntBig,
            Type::String,
            Type::StringSized(2),
            Type::Text,
            Type::Float,
            Type::FloatBig,
            Type::Binary,
            Type::BinarySized(2),
            Type::Uuid,
            Type::Raw("BLA".to_string()),
        ];
        for syntax in &syntaxes {
            for ty in &types {
                let rt = ty.rust_type();
                eprintln!("{:?}, {:?}, {:?}", syntax, rt, ty);
                let _ = ty.db_type(*syntax);
                let _ = ty.db_id_type(*syntax);
            }
        }
    }
}
