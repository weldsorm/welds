use crate::config::DbProvider;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) struct TypeInfo {
    pub quote: TokenStream,
    pub force_null: bool,
}

impl TypeInfo {
    fn new(mut quote: TokenStream, array: bool) -> TypeInfo {
        if array {
            quote = quote!(Vec<#quote>);
        }
        TypeInfo {
            quote,
            force_null: false,
        }
    }

    fn force_null(mut quote: TokenStream, array: bool) -> TypeInfo {
        if array {
            quote = quote!(Vec<#quote>);
        }
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

fn get_mssql(mut db_type: &str) -> Option<TypeInfo> {
    let mut array = false;
    if db_type.ends_with("[]") {
        array = true;
        db_type = &db_type[..db_type.len() - 2];
    }

    let ty = match db_type {
        "BIT" => TypeInfo::new(quote!(i32), array),
        "INT" => TypeInfo::new(quote!(i32), array),
        "BIGINT" => TypeInfo::new(quote!(i64), array),
        "REAL" => TypeInfo::new(quote!(f32), array),
        "VARCHAR" => TypeInfo::new(quote!(String), array),
        _ => return None,
    };
    Some(ty)

    // TODO: make this a full list
}

fn get_mysql(mut db_type: &str) -> Option<TypeInfo> {
    let mut array = false;
    if db_type.ends_with("[]") {
        array = true;
        db_type = &db_type[..db_type.len() - 2];
    }

    let ty = match db_type {
        "TINYINT(1)" => TypeInfo::new(quote!(bool), array),
        "BOOLEAN" => TypeInfo::new(quote!(bool), array),
        "TINYINT" => TypeInfo::new(quote!(i8), array),
        "SMALLINT" => TypeInfo::new(quote!(i16), array),
        "INT" => TypeInfo::new(quote!(i32), array),
        "BIGINT" => TypeInfo::new(quote!(i64), array),
        "TINYINT UNSIGNED" => TypeInfo::new(quote!(u8), array),
        "SMALLINT UNSIGNED" => TypeInfo::new(quote!(u16), array),
        "INT UNSIGNED" => TypeInfo::new(quote!(u32), array),
        "BIGINT UNSIGNED" => TypeInfo::new(quote!(u64), array),
        "FLOAT" => TypeInfo::new(quote!(f32), array),
        "DOUBLE" => TypeInfo::new(quote!(f64), array),
        "VARCHAR" => TypeInfo::new(quote!(String), array),
        "CHAR " => TypeInfo::new(quote!(String), array),
        "TEXT" => TypeInfo::new(quote!(String), array),
        "VARBINARY" => TypeInfo::new(quote!(Vec<u8>), array),
        "BINARY" => TypeInfo::new(quote!(Vec<u8>), array),
        "BLOB" => TypeInfo::new(quote!(Vec<u8>), array),
        "TIMESTAMP" => TypeInfo::new(quote!(chrono::DateTime<chrono::Utc>), array),
        "DATETIME" => TypeInfo::new(quote!(chrono::NaiveDateTime), array),
        "DATE" => TypeInfo::new(quote!(chrono::NaiveDate), array),
        "TIME" => TypeInfo::new(quote!(chrono::NaiveTime), array),
        _ => return None,
    };
    Some(ty)
}

fn get_sqlite(mut db_type: &str) -> Option<TypeInfo> {
    let mut array = false;
    if db_type.ends_with("[]") {
        array = true;
        db_type = &db_type[..db_type.len() - 2];
    }

    let ty = match db_type {
        "BOOLEAN" => TypeInfo::new(quote!(bool), array),
        "INTEGER" => TypeInfo::new(quote!(i32), array),
        "BIGINT" => TypeInfo::new(quote!(i64), array),
        "INT8" => TypeInfo::new(quote!(i64), array),
        "REAL" => TypeInfo::new(quote!(f64), array),
        "TEXT" => TypeInfo::new(quote!(String), array),
        "BLOB" => TypeInfo::new(quote!(Vec<u8>), array),
        "DATETIME" => TypeInfo::new(quote!(chrono::DateTime<chrono::Utc>), array),
        "DATE" => TypeInfo::new(quote!(chrono::NaiveDate), array),
        "TIME" => TypeInfo::new(quote!(chrono::NaiveTime), array),
        _ => return None,
    };
    Some(ty)
}

fn get_postgres(mut db_type: &str) -> Option<TypeInfo> {
    let mut array = false;
    if db_type.ends_with("[]") {
        array = true;
        db_type = &db_type[..db_type.len() - 2];
    }

    let ty = match db_type {
        "BOOL" => TypeInfo::new(quote!(bool), array),
        "CHAR" => TypeInfo::new(quote!(i8), array),
        "INT2" => TypeInfo::new(quote!(i16), array),
        "SMALLINT" => TypeInfo::new(quote!(i16), array),
        "SMALLSERIAL" => TypeInfo::new(quote!(i16), array),
        "INT4" => TypeInfo::new(quote!(i32), array),
        "SERIAL" => TypeInfo::new(quote!(i32), array),
        "INT" => TypeInfo::new(quote!(i32), array),
        "BIGINT" => TypeInfo::new(quote!(i64), array),
        "INT8" => TypeInfo::new(quote!(i64), array),
        "BIGSERIAL" => TypeInfo::new(quote!(i64), array),
        "REAL" => TypeInfo::new(quote!(f32), array),
        "FLOAT4" => TypeInfo::new(quote!(f32), array),
        "DOUBLE PRECISION" => TypeInfo::new(quote!(f64), array),
        "FLOAT8" => TypeInfo::new(quote!(f64), array),
        "VARCHAR" => TypeInfo::new(quote!(String), array),
        "CHAR(N)" => TypeInfo::new(quote!(String), array),
        "TEXT" => TypeInfo::new(quote!(String), array),
        "NAME" => TypeInfo::new(quote!(String), array),
        "BYTEA" => TypeInfo::new(quote!(Vec<u8>), array),
        "BLOB" => TypeInfo::new(quote!(Vec<u8>), array),
        "INTERVAL" => TypeInfo::force_null(quote!(sqlx::postgres::types::PgInterval), array),
        "MONEY" => TypeInfo::force_null(quote!(sqlx::postgres::types::PgMoney), array),
        "INT4RANGE" => TypeInfo::force_null(quote!(sqlx::postgres::types::PgRange<i32>), array),
        "INT8RANGE" => TypeInfo::force_null(quote!(sqlx::postgres::types::PgRange<i64>), array),
        "TIMESTAMPTZ" => TypeInfo::force_null(quote!(chrono::DateTime<chrono::Utc>), array),
        "TIMESTAMP" => TypeInfo::force_null(quote!(chrono::NaiveDateTime), array),
        "DATE" => TypeInfo::force_null(quote!(chrono::NaiveDate), array),
        "TIME" => TypeInfo::force_null(quote!(chrono::NaiveTime), array),
        "TIMETZ" => TypeInfo::force_null(quote!(sqlx::postgres::types::PgTimeTz), array),

        _ => return None,
    };
    Some(ty)
}
