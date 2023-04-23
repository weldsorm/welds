use crate::config::DbProvider;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) struct TypeInfo {
    pub quote: TokenStream,
    pub force_null: bool,
}

impl TypeInfo {
    fn new(quote: TokenStream) -> TypeInfo {
        TypeInfo {
            quote,
            force_null: false,
        }
    }
    fn force_null(quote: TokenStream) -> TypeInfo {
        TypeInfo {
            quote,
            force_null: true,
        }
    }
}

/// This is a collection of which rust types to use for a given DB type
pub(crate) fn get(db_type: &str, db: DbProvider) -> Option<TypeInfo> {
    let db_type = db_type.to_uppercase();
    match db {
        DbProvider::Mysql => get_mysql(&db_type),
        DbProvider::Mssql => get_mssql(&db_type),
        DbProvider::Postgres => get_postgres(&db_type),
        DbProvider::Sqlite => get_sqlite(&db_type),
    }
}

fn get_mssql(db_type: &str) -> Option<TypeInfo> {
    let ty = match db_type {
        "BIT" => TypeInfo::new(quote!(i32)),
        "INT" => TypeInfo::new(quote!(i32)),
        "BIGINT" => TypeInfo::new(quote!(i64)),
        "REAL" => TypeInfo::new(quote!(f32)),
        "VARCHAR" => TypeInfo::new(quote!(String)),
        _ => return None,
    };
    Some(ty)

    // TODO: make this a full list
}

fn get_mysql(db_type: &str) -> Option<TypeInfo> {
    let ty = match db_type {
        "TINYINT(1)" => TypeInfo::new(quote!(bool)),
        "BOOLEAN" => TypeInfo::new(quote!(bool)),
        "TINYINT" => TypeInfo::new(quote!(i8)),
        "SMALLINT" => TypeInfo::new(quote!(i16)),
        "INT" => TypeInfo::new(quote!(i32)),
        "BIGINT" => TypeInfo::new(quote!(i64)),
        "TINYINT UNSIGNED" => TypeInfo::new(quote!(u8)),
        "SMALLINT UNSIGNED" => TypeInfo::new(quote!(u16)),
        "INT UNSIGNED" => TypeInfo::new(quote!(u32)),
        "BIGINT UNSIGNED" => TypeInfo::new(quote!(u64)),
        "FLOAT" => TypeInfo::new(quote!(f32)),
        "DOUBLE" => TypeInfo::new(quote!(f64)),
        "VARCHAR" => TypeInfo::new(quote!(String)),
        "CHAR " => TypeInfo::new(quote!(String)),
        "TEXT" => TypeInfo::new(quote!(String)),
        "VARBINARY" => TypeInfo::new(quote!(Vec<u8>)),
        "BINARY" => TypeInfo::new(quote!(Vec<u8>)),
        "BLOB" => TypeInfo::new(quote!(Vec<u8>)),
        "TIMESTAMP" => TypeInfo::new(quote!(chrono::DateTime<chrono::Utc>)),
        "DATETIME" => TypeInfo::new(quote!(chrono::NaiveDateTime)),
        "DATE" => TypeInfo::new(quote!(chrono::NaiveDate)),
        "TIME" => TypeInfo::new(quote!(chrono::NaiveTime)),
        _ => return None,
    };
    Some(ty)
}

fn get_sqlite(db_type: &str) -> Option<TypeInfo> {
    let ty = match db_type {
        "BOOLEAN" => TypeInfo::new(quote!(bool)),
        "INTEGER" => TypeInfo::new(quote!(i32)),
        "BIGINT" => TypeInfo::new(quote!(i64)),
        "INT8" => TypeInfo::new(quote!(i64)),
        "REAL" => TypeInfo::new(quote!(f64)),
        "TEXT" => TypeInfo::new(quote!(String)),
        "BLOB" => TypeInfo::new(quote!(Vec<u8>)),
        "DATETIME" => TypeInfo::new(quote!(chrono::DateTime<chrono::Utc>)),
        "DATE" => TypeInfo::new(quote!(chrono::NaiveDate)),
        "TIME" => TypeInfo::new(quote!(chrono::NaiveTime)),
        _ => return None,
    };
    Some(ty)
}

fn get_postgres(db_type: &str) -> Option<TypeInfo> {
    let ty = match db_type {
        "BOOL" => TypeInfo::new(quote!(bool)),
        "CHAR" => TypeInfo::new(quote!(i8)),
        "INT2" => TypeInfo::new(quote!(i16)),
        "SMALLINT" => TypeInfo::new(quote!(i16)),
        "SMALLSERIAL" => TypeInfo::new(quote!(i16)),
        "INT4" => TypeInfo::new(quote!(i32)),
        "SERIAL" => TypeInfo::new(quote!(i32)),
        "INT" => TypeInfo::new(quote!(i32)),
        "BIGINT" => TypeInfo::new(quote!(i64)),
        "INT8" => TypeInfo::new(quote!(i64)),
        "BIGSERIAL" => TypeInfo::new(quote!(i64)),
        "REAL" => TypeInfo::new(quote!(f32)),
        "FLOAT4" => TypeInfo::new(quote!(f32)),
        "DOUBLE PRECISION" => TypeInfo::new(quote!(f64)),
        "FLOAT8" => TypeInfo::new(quote!(f64)),
        "VARCHAR" => TypeInfo::new(quote!(String)),
        "CHAR(N)" => TypeInfo::new(quote!(String)),
        "TEXT" => TypeInfo::new(quote!(String)),
        "NAME" => TypeInfo::new(quote!(String)),
        "BYTEA" => TypeInfo::new(quote!(Vec<u8>)),
        "BLOB" => TypeInfo::new(quote!(Vec<u8>)),
        "INTERVAL" => TypeInfo::force_null(quote!(sqlx::postgres::types::PgInterval)),
        "MONEY" => TypeInfo::force_null(quote!(sqlx::postgres::types::PgMoney)),
        "INT4RANGE" => TypeInfo::force_null(quote!(sqlx::postgres::types::PgRange<i32>)),
        "INT8RANGE" => TypeInfo::force_null(quote!(sqlx::postgres::types::PgRange<i64>)),
        "TIMESTAMPTZ" => TypeInfo::force_null(quote!(chrono::DateTime<chrono::Utc>)),
        "TIMESTAMP" => TypeInfo::force_null(quote!(chrono::NaiveDateTime)),
        "DATE" => TypeInfo::force_null(quote!(chrono::NaiveDate)),
        "TIME" => TypeInfo::force_null(quote!(chrono::NaiveTime)),
        "TIMETZ" => TypeInfo::force_null(quote!(sqlx::postgres::types::PgTimeTz)),

        _ => return None,
    };
    Some(ty)
}
