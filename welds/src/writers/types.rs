use crate::Syntax;

#[derive(Debug, Clone)]
pub struct Pair {
    db_type: &'static str,
    rust_type: &'static str,
    // if true this pair shouldn't be used in a migration unless it is a PK
    id_only: bool,
    // If true a size is required to make this type in the DB
    sized: bool,
    // If true a size is required to make this type in the DB
    default_size: Option<&'static str>,
    // match on the array version of this pair not the two types
    array: bool,
}

impl Pair {
    const fn new(db_type: &'static str, rust_type: &'static str) -> Pair {
        Pair {
            db_type,
            rust_type,
            id_only: false,
            sized: false,
            default_size: None,
            array: false,
        }
    }

    /// Used to say the db type includes a size value.
    /// This is important to know when checking if the
    /// DB type matches the models fields
    const fn sized(
        db_type: &'static str,
        rust_type: &'static str,
        default_size: &'static str,
    ) -> Pair {
        Pair {
            db_type,
            rust_type,
            id_only: false,
            sized: true,
            default_size: Some(default_size),
            array: false,
        }
    }

    /// Used to to say this pair should only be used on ID fields
    /// This way the migrations will not pick this match unless
    /// it is looking for a primary key type
    const fn key_only(db_type: &'static str, rust_type: &'static str) -> Pair {
        Pair {
            db_type,
            rust_type,
            id_only: true,
            sized: false,
            default_size: None,
            array: false,
        }
    }

    /// returns true is this Pair should ONLY be used in a PK column type.
    pub fn id_only(&self) -> bool {
        self.id_only
    }

    // returns true if the DB type of this pair requires a size. Example: VARCHAR(255)
    pub fn db_sized(&self) -> bool {
        self.sized
    }

    // Returns a reasonable default value to use with a given DB type if it is size.
    // This is used when creating migrations and the use don't pick a size
    pub fn default_size(&self) -> Option<&'static str> {
        self.default_size
    }

    pub fn db_type(&self) -> String {
        if self.array {
            format!("{}[]", self.db_type)
        } else {
            self.db_type.to_owned()
        }
    }

    pub fn rust_type(&self) -> String {
        let base = match self.rust_type.rfind(':') {
            Some(index) => &self.rust_type[index + 1..],
            None => self.rust_type,
        };
        if self.array {
            format!("Vec<{}>", base)
        } else {
            base.to_owned()
        }
    }

    /// Returns a full rust type with a namespace if there is one
    pub fn full_rust_type(&self) -> String {
        if self.array {
            format!("Vec<{}>", self.rust_type)
        } else {
            self.rust_type.to_owned()
        }
    }

    /// returns true is this Pair matches a db_type and rust_type
    pub fn matches(&self, db: &str, rust: &str) -> bool {
        self.db_type() == db && self.is_rust_type(rust)
    }

    /// Returns true the Pair matches a given rust type
    pub fn is_rust_type(&self, rust: &str) -> bool {
        let self_rust_type = self.rust_type();
        if self_rust_type == rust {
            return true;
        }
        // ignoring the namespace
        let rust_typeonly = match rust.rfind(':') {
            Some(index) => &rust[index + 1..],
            None => rust,
        };
        if self_rust_type == rust_typeonly {
            return true;
        }
        // not a match
        false
    }
}

/// Returns a list of DB_TYPE and RUST_TYPE pairs.
/// A pair can be assumed to be usable together in welds
/// I.E.  INT <=> i32
/// a model with a type i32 will work with a db_type of INT
pub fn get_pairs(syntax: Syntax) -> Vec<Pair> {
    let base_pairs = get_basic_type_pairs(syntax);
    let arrays = base_pairs.iter().map(|p| {
        let mut a = p.clone();
        a.array = true;
        a
    });
    base_pairs.iter().cloned().chain(arrays).collect()
}

/// Same as get_pairs but doesn't include arrays types
/// INT[] <=> Vec<i32>
pub fn get_basic_type_pairs(syntax: Syntax) -> &'static [Pair] {
    match syntax {
        Syntax::Postgres => POSTGRES_PAIRS,
        Syntax::Sqlite => SQLITE_PAIRS,
        Syntax::Mysql => MYSQL_PAIRS,
        Syntax::Mssql => MSSQL_PAIRS,
    }
}

/***************************************************************************
 *
 * WARNING: ORDER MATTERS!!!
 * these pairs of type are used to pick the best type when making migrations
 * The high up in the list, it will get match first.
 *
 * **********************************************************************/

const MSSQL_PAIRS: &[Pair] = &[
    Pair::new("INT", "i32"),
    Pair::new("BIT", "bool"),
    Pair::new("SMALLINT", "i16"),
    Pair::new("BIGINT", "i64"),
    Pair::new("FLOAT(24)", "f32"),
    Pair::new("FLOAT(53)", "f64"),
    Pair::sized("NVARCHAR", "String", "MAX"),
    Pair::sized("VARCHAR", "String", "MAX"),
    Pair::new("TEXT", "String"),
    Pair::sized("VARBINARY", "Vec<u8>", "MAX"),
    Pair::new("UNIQUEIDENTIFIER", "Uuid"),
    Pair::new("JSON", "serde_json::Value"),
];

const MYSQL_PAIRS: &[Pair] = &[
    Pair::new("TINYINT(1)", "bool"),
    Pair::new("BOOLEAN", "bool"),
    Pair::new("TINYINT", "i8"),
    Pair::new("SMALLINT", "i16"),
    Pair::new("INT", "i32"),
    Pair::new("BIGINT", "i64"),
    Pair::new("TINYINT UNSIGNED", "u8"),
    Pair::new("SMALLINT UNSIGNED", "u16"),
    Pair::new("INT UNSIGNED", "u32"),
    Pair::new("BIGINT UNSIGNED", "u64"),
    Pair::new("FLOAT", "f32"),
    Pair::new("DOUBLE", "f64"),
    Pair::new("VARCHAR(255)", "String"),
    Pair::new("TEXT", "String"),
    Pair::sized("VARCHAR", "String", "255"),
    Pair::sized("CHAR ", "String", "255"),
    Pair::new("BLOB", "Vec<u8>"),
    Pair::new("TINYBLOB", "Vec<u8>"),
    Pair::new("MEDIUMBLOB", "Vec<u8>"),
    Pair::new("LONGBLOB", "Vec<u8>"),
    Pair::sized("BINARY", "Vec<u8>", "255"),
    Pair::sized("VARBINARY", "Vec<u8>", "255"),
    Pair::new("TIMESTAMP", "chrono::DateTime<chrono::Utc>"),
    Pair::new("DATETIME", "chrono::NaiveDateTime"),
    Pair::new("DATE", "chrono::NaiveDate"),
    Pair::new("TIME", "chrono::NaiveTime"),
    Pair::new("VARCHAR(36)", "sqlx::types::Uuid"),
    Pair::new("JSON", "serde_json::Value"),
];

const SQLITE_PAIRS: &[Pair] = &[
    Pair::new("BOOLEAN", "bool"),
    Pair::new("BOOL", "bool"),
    Pair::new("INTEGER", "i32"),
    Pair::new("INTEGER", "i64"),
    Pair::new("BIGINT", "i64"),
    Pair::new("INT8", "i64"),
    Pair::new("INT2", "i64"),
    Pair::new("INT", "i64"),
    Pair::new("TINYINT", "i64"),
    Pair::new("SMALLINT", "i64"),
    Pair::new("MEDIUMINT", "i64"),
    Pair::new("UNSIGNED BIG INT", "i64"),
    Pair::new("REAL", "f64"),
    Pair::new("DOUBLE", "f64"),
    Pair::new("DOUBLE PRECISION", "f64"),
    Pair::new("FLOAT", "f64"),
    Pair::new("TEXT", "String"),
    Pair::new("CHAR", "String"),
    Pair::new("VARCHAR", "String"),
    Pair::new("VARYING CHARACTER", "String"),
    Pair::new("NCHAR", "String"),
    Pair::new("NATIVE CHARACTER", "String"),
    Pair::new("NVARCHAR", "String"),
    Pair::new("CLOB", "String"),
    Pair::new("BLOB", "Vec<u8>"),
    Pair::new("NUMERIC", "rust_decimal::Decimal"),
    Pair::new("DECIMAL", "rust_decimal::Decimal"),
    Pair::new("DATETIME", "chrono::DateTime<chrono::Utc>"),
    Pair::new("DATETIME", "chrono::DateTime<Utc>"),
    Pair::new("DATE", "chrono::NaiveDate"),
    Pair::new("TIME", "chrono::NaiveTime"),
    // These Pairs are here so all rust int types have a match
    Pair::new("INTEGER", "i16"),
    Pair::new("INTEGER", "i32"),
    Pair::new("REAL", "f32"),
    Pair::new("TEXT", "sqlx::types::Uuid"),
    Pair::new("TEXT", "serde_json::Value"),
];

const POSTGRES_PAIRS: &[Pair] = &[
    Pair::key_only("SERIAL", "i32"),
    Pair::key_only("BIGSERIAL", "i64"),
    Pair::key_only("SMALLSERIAL", "i16"),
    Pair::new("UUID", "sqlx::types::Uuid"),
    Pair::new("BOOL", "bool"),
    Pair::new("CHAR", "i8"), // not sized, single char
    Pair::new("INT2", "i16"),
    Pair::new("SMALLINT", "i16"),
    Pair::new("INT", "i32"),
    Pair::new("INT4", "i32"),
    Pair::new("BIGINT", "i64"),
    Pair::new("INT8", "i64"),
    Pair::new("REAL", "f32"),
    Pair::new("FLOAT4", "f32"),
    Pair::new("DOUBLE PRECISION", "f64"),
    Pair::new("FLOAT8", "f64"),
    Pair::new("TEXT", "String"),
    Pair::sized("VARCHAR", "String", "255"),
    Pair::sized("CHAR", "String", "255"),
    Pair::new("NAME", "String"),
    Pair::new("BYTEA", "Vec<u8>"),
    Pair::new("BLOB", "Vec<u8>"),
    Pair::new("INTERVAL", "sqlx::postgres::types::PgInterval"),
    Pair::new("MONEY", "sqlx::postgres::types::PgMoney"),
    Pair::new("INT4RANGE", "sqlx::postgres::types::PgRange<i32>"),
    Pair::new("INT8RANGE", "sqlx::postgres::types::PgRange<i64>"),
    Pair::new("TIMESTAMPTZ", "chrono::DateTime<chrono::Utc>"),
    Pair::new("TIMESTAMPTZ", "chrono::DateTime<Utc>"),
    Pair::new("TIMESTAMP", "chrono::NaiveDateTime"),
    Pair::new("DATE", "chrono::NaiveDate"),
    Pair::new("TIME", "chrono::NaiveTime"),
    Pair::new("TIMETZ", "sqlx::postgres::types::PgTimeTz"),
    Pair::new("JSONB", "serde_json::Value"),
];

/// Returns true if two types are a match
pub(crate) fn are_equivalent_types(pairs: &[Pair], db: &str, rust: &str) -> bool {
    let db = db.trim().to_uppercase();
    for pair in pairs {
        if pair.matches(&db, rust) {
            return true;
        }
    }
    false
}

/// Override the type of a column to the type that should be used
/// to crate its PK value. e.g.
pub(crate) fn pk_override(syntax: Syntax, db_type: &str) -> Option<&'static str> {
    // Use the Serial type to create the PKs, the type will be reported back as int..
    if let Syntax::Postgres = syntax {
        match db_type {
            "INT2" => return Some("SMALLSERIAL"),
            "SMALLINT" => return Some("SMALLSERIAL"),
            "INT" => return Some("SERIAL"),
            "INT4" => return Some("SERIAL"),
            "BIGINT" => return Some("BIGSERIAL"),
            "INT8" => return Some("BIGSERIAL"),
            _ => {}
        }
    }
    // Sqlite uses dynamic sizes for int, should always be INTEGER in pk create
    if let Syntax::Sqlite = syntax {
        match db_type {
            "INT4" => return Some("INTEGER"),
            "INT8" => return Some("INTEGER"),
            "INT" => return Some("INTEGER"),
            "BIGINT" => return Some("INTEGER"),
            "INTSMALL" => return Some("INTEGER"),
            _ => {}
        }
    }
    None
}

/// Returns the recommenced rust type to use for a given Database type.
pub fn recommended_rust_type(syntax: Syntax, db_type: &str) -> Option<Pair> {
    // find the root of the type VARCHAR from VARCHAR(MAX)
    let base = match db_type.find('(') {
        Some(index) => &db_type[..index],
        None => db_type,
    };

    let pairs = get_pairs(syntax);
    let db_type = db_type.trim().to_uppercase();
    for pair in pairs {
        let pair_type = pair.db_type();
        if base == pair_type || db_type == pair_type {
            return Some(pair);
        }
    }
    None
}

/// Returns the recommended rust type to use for a given Database type.
pub fn recommended_db_type(syntax: Syntax, rust_type: &str) -> Option<Pair> {
    let pairs = get_pairs(syntax);
    let pairs = pairs.iter().filter(|&x| !x.id_only());
    let ty = rust_type.trim();
    for pair in pairs {
        if pair.is_rust_type(ty) {
            return Some(pair.clone());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equivalent_types() {
        let pairs = get_pairs(Syntax::Postgres);
        assert!(!are_equivalent_types(&pairs, "INT4", "i64"));
        assert!(are_equivalent_types(&pairs, "INT4", "i32"));
        assert!(!are_equivalent_types(&pairs, "SERIAL", "String"));
        assert!(are_equivalent_types(&pairs, "BIGSERIAL", "i64"));
        assert!(are_equivalent_types(&pairs, "BIGINT[]", "Vec<i64>"));
        assert!(!are_equivalent_types(&pairs, "BIGINT", "Vec<i64>"));
        assert!(are_equivalent_types(&pairs, "VARCHAR", "String"));
        assert!(!are_equivalent_types(&pairs, "FLOAT", "String"));
        assert!(are_equivalent_types(&pairs, "MONEY", "PgMoney"));
    }

    #[test]
    fn should_recommend_good_rust_types() {
        let s = Syntax::Postgres;
        assert_eq!(
            recommended_rust_type(s, "Int").unwrap().full_rust_type(),
            "i32"
        );
        assert_eq!(
            recommended_rust_type(s, "text").unwrap().full_rust_type(),
            "String"
        );
        assert_eq!(
            recommended_rust_type(s, "bool").unwrap().full_rust_type(),
            "bool"
        );
        let s = Syntax::Sqlite;
        assert_eq!(
            recommended_rust_type(s, "SMALLINT")
                .unwrap()
                .full_rust_type(),
            "i64"
        );
        assert_eq!(
            recommended_rust_type(s, "BIGINT").unwrap().full_rust_type(),
            "i64"
        );
        let s = Syntax::Mssql;
        assert_eq!(
            recommended_rust_type(s, "VARCHAR(MAX)")
                .unwrap()
                .full_rust_type(),
            "String"
        );
        assert_eq!(
            recommended_rust_type(s, "TEXT").unwrap().full_rust_type(),
            "String"
        );
    }

    #[test]
    fn should_recommend_good_db_types() {
        let s = Syntax::Postgres;
        assert_eq!(recommended_db_type(s, "i32").unwrap().db_type(), "INT");
        assert_eq!(recommended_db_type(s, "String").unwrap().db_type(), "TEXT");
        assert_eq!(recommended_db_type(s, "bool").unwrap().db_type(), "BOOL");
        let s = Syntax::Sqlite;
        assert_eq!(recommended_db_type(s, "i64").unwrap().db_type(), "INTEGER");
        assert_eq!(recommended_db_type(s, "i16").unwrap().db_type(), "INTEGER");
    }
}
