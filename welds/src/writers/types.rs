use crate::Syntax;

#[derive(Debug, Clone)]
pub struct Pair {
    db_type: String,
    rust_type: String,
}
impl Pair {
    fn new(db_type: &'static str, rust_type: &'static str) -> Pair {
        Pair {
            db_type: db_type.to_string(),
            rust_type: rust_type.to_string(),
        }
    }
    pub fn db_type(&self) -> &str {
        &self.db_type
    }

    pub fn rust_type(&self) -> &str {
        match self.rust_type.rfind(':') {
            Some(index) => &self.rust_type.as_str()[index + 1..],
            None => &self.rust_type,
        }
    }

    pub fn matches(&self, db: &str, rust: &str) -> bool {
        if self.db_type != db {
            return false;
        }

        if self.rust_type == rust {
            return true;
        }

        let rust_typeonly = match rust.rfind(':') {
            Some(index) => &self.rust_type.as_str()[index + 1..],
            None => rust,
        };

        if self.rust_type() == rust_typeonly {
            return true;
        }

        false
    }
}

pub fn get_pairs(syntax: Syntax) -> Vec<Pair> {
    let base_pairs = match syntax {
        Syntax::Sqlite => sqlite_pairs(),
        Syntax::Postgres => postgres_pairs(),
        Syntax::Mysql => mysql_pairs(),
        Syntax::Mssql => mssql_pairs(),
    };
    let arrays = base_pairs.iter().map(|p| Pair {
        db_type: format!("{}[]", p.db_type),
        rust_type: format!("Vec<{}>", p.rust_type()),
    });
    base_pairs.iter().cloned().chain(arrays).collect()
}

fn mssql_pairs() -> Vec<Pair> {
    vec![
        Pair::new("BIT", "i32"),
        Pair::new("INT", "i32"),
        Pair::new("BIGINT", "i64"),
        Pair::new("REAL", "f32"),
        Pair::new("VARCHAR", "String"),
        Pair::new("TEXT", "String"),
    ]
}

fn mysql_pairs() -> Vec<Pair> {
    vec![
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
        Pair::new("VARCHAR", "String"),
        Pair::new("CHAR ", "String"),
        Pair::new("TEXT", "String"),
        Pair::new("VARBINARY", "Vec<u8>"),
        Pair::new("BINARY", "Vec<u8>"),
        Pair::new("BLOB", "Vec<u8>"),
        Pair::new("TIMESTAMP", "chrono::DateTime<chrono::Utc>"),
        Pair::new("DATETIME", "chrono::NaiveDateTime"),
        Pair::new("DATE", "chrono::NaiveDate"),
        Pair::new("TIME", "chrono::NaiveTime"),
    ]
}

fn sqlite_pairs() -> Vec<Pair> {
    vec![
        Pair::new("BOOLEAN", "bool"),
        Pair::new("INTEGER", "i32"),
        Pair::new("BIGINT", "i64"),
        Pair::new("INT8", "i64"),
        Pair::new("REAL", "f64"),
        Pair::new("TEXT", "String"),
        Pair::new("BLOB", "Vec<u8>"),
        Pair::new("DATETIME", "chrono::DateTime<chrono::Utc>"),
        Pair::new("DATETIME", "chrono::DateTime<Utc>"),
        Pair::new("DATE", "chrono::NaiveDate"),
        Pair::new("TIME", "chrono::NaiveTime"),
    ]
}

fn postgres_pairs() -> Vec<Pair> {
    vec![
        Pair::new("BOOL", "bool"),
        Pair::new("CHAR", "i8"),
        Pair::new("INT2", "i16"),
        Pair::new("SMALLINT", "i16"),
        Pair::new("SMALLSERIAL", "i16"),
        Pair::new("INT", "i32"),
        Pair::new("INT4", "i32"),
        Pair::new("SERIAL", "i32"),
        Pair::new("BIGINT", "i64"),
        Pair::new("INT8", "i64"),
        Pair::new("BIGSERIAL", "i64"),
        Pair::new("REAL", "f32"),
        Pair::new("FLOAT4", "f32"),
        Pair::new("DOUBLE PRECISION", "f64"),
        Pair::new("FLOAT8", "f64"),
        Pair::new("VARCHAR", "String"),
        Pair::new("CHAR(N)", "String"),
        Pair::new("TEXT", "String"),
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
    ]
}
