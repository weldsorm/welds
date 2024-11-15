use crate::writers::types::get_pairs;
use crate::writers::types::Pair;
use crate::Syntax;
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
    Uuid,
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
            Type::Uuid => "Uuid".to_owned(),
            Type::Raw(_) => return None,
        })
    }

    /// Returns the DB type to use in a migration
    pub fn db_type(&self, syntax: Syntax) -> String {
        let pairs = get_pairs(syntax);
        let mut pairs_iter = pairs.iter().filter(|x| !x.id_only());
        find_db_type(self, &mut pairs_iter)
    }

    /// Returns the DB type to use in a migration for Id columns
    pub fn db_id_type(&self, syntax: Syntax) -> String {
        let pairs = get_pairs(syntax);
        let mut pairs_iter = pairs.iter();
        find_db_type(self, &mut pairs_iter)
    }

    /// Reads in a DB type and returns a type version of it.
    pub(crate) fn parse_db_type(syntax: Syntax, db_type: &str) -> Type {
        let db_type = db_type.to_uppercase();
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
fn find_db_type<'i, I>(ty: &Type, pairs: &mut I) -> String
where
    I: Iterator<Item = &'i Pair>,
{
    // If the developer want to give us a type use it
    if let Type::Raw(raw) = ty {
        return raw.to_owned();
    }
    // find the best DB type based on the rust type
    let rust_type = ty.rust_type().unwrap();
    let pair = pairs.find(|&p| p.is_rust_type(&rust_type)).unwrap();
    // If the pair is sized. Make sure we add the size info
    if pair.db_sized() {
        let size = match ty {
            Type::StringSized(s) => s.to_string(),
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
