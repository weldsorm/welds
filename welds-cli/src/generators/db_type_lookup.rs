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
    match db {
        DbProvider::Mysql => get_mysql(db_type),
        DbProvider::Mssql => get_mssql(db_type),
        DbProvider::Postgres => get_postgres(db_type),
        DbProvider::Sqlite => get_sqlite(db_type),
    }
}

fn get_mssql(db_type: &str) -> Option<TypeInfo> {
    let ty = match db_type {
        "bit" => TypeInfo::new(quote!(i32)),
        "int" => TypeInfo::new(quote!(i32)),
        "bigint" => TypeInfo::new(quote!(i64)),
        "real" => TypeInfo::new(quote!(f32)),
        "varchar" => TypeInfo::new(quote!(String)),
        _ => return None,
    };
    Some(ty)

    // TODO: make this a full list
}

fn get_mysql(db_type: &str) -> Option<TypeInfo> {
    let ty = match db_type {
        "tinyint(1)" => TypeInfo::new(quote!(bool)),
        "boolean" => TypeInfo::new(quote!(bool)),
        "tinyint" => TypeInfo::new(quote!(i8)),
        "smallint" => TypeInfo::new(quote!(i16)),
        "int" => TypeInfo::new(quote!(i32)),
        "bigint" => TypeInfo::new(quote!(i64)),
        "tinyint unsigned" => TypeInfo::new(quote!(u8)),
        "smallint unsigned" => TypeInfo::new(quote!(u16)),
        "int unsigned" => TypeInfo::new(quote!(u32)),
        "bigint unsigned" => TypeInfo::new(quote!(u64)),
        "float" => TypeInfo::new(quote!(f32)),
        "double" => TypeInfo::new(quote!(f64)),
        "varchar" => TypeInfo::new(quote!(String)),
        "char " => TypeInfo::new(quote!(String)),
        "text" => TypeInfo::new(quote!(String)),
        "varbinary" => TypeInfo::new(quote!(Vec<u8>)),
        "binary" => TypeInfo::new(quote!(Vec<u8>)),
        "blob" => TypeInfo::new(quote!(Vec<u8>)),
        "timestamp" => TypeInfo::new(quote!(chrono::DateTime<chrono::Utc>)),
        "datetime" => TypeInfo::new(quote!(chrono::NaiveDateTime)),
        "date" => TypeInfo::new(quote!(chrono::NaiveDate)),
        "time" => TypeInfo::new(quote!(chrono::NaiveTime)),
        _ => return None,
    };
    Some(ty)
}

fn get_sqlite(db_type: &str) -> Option<TypeInfo> {
    let ty = match db_type {
        "boolean" => TypeInfo::new(quote!(bool)),
        "integer" => TypeInfo::new(quote!(i32)),
        "bigint" => TypeInfo::new(quote!(i64)),
        "int8" => TypeInfo::new(quote!(i64)),
        "real" => TypeInfo::new(quote!(f64)),
        "text" => TypeInfo::new(quote!(String)),
        "blob" => TypeInfo::new(quote!(Vec<u8>)),
        "datetime" => TypeInfo::new(quote!(chrono::DateTime<chrono::Utc>)),
        "date" => TypeInfo::new(quote!(chrono::NaiveDate)),
        "time" => TypeInfo::new(quote!(chrono::NaiveTime)),
        _ => return None,
    };
    Some(ty)
}

fn get_postgres(db_type: &str) -> Option<TypeInfo> {
    let ty = match db_type {
        "bool" => TypeInfo::new(quote!(bool)),
        "char" => TypeInfo::new(quote!(i8)),
        "int2" => TypeInfo::new(quote!(i16)),
        "smallint" => TypeInfo::new(quote!(i16)),
        "smallserial" => TypeInfo::new(quote!(i16)),
        "int4" => TypeInfo::new(quote!(i32)),
        "serial" => TypeInfo::new(quote!(i32)),
        "int" => TypeInfo::new(quote!(i32)),
        "bigint" => TypeInfo::new(quote!(i64)),
        "int8" => TypeInfo::new(quote!(i64)),
        "bigserial" => TypeInfo::new(quote!(i64)),
        "real" => TypeInfo::new(quote!(f32)),
        "float4" => TypeInfo::new(quote!(f32)),
        "double precision" => TypeInfo::new(quote!(f64)),
        "float8" => TypeInfo::new(quote!(f64)),
        "varchar" => TypeInfo::new(quote!(String)),
        "char(n)" => TypeInfo::new(quote!(String)),
        "text" => TypeInfo::new(quote!(String)),
        "name" => TypeInfo::new(quote!(String)),
        "bytea" => TypeInfo::new(quote!(Vec<u8>)),
        "blob" => TypeInfo::new(quote!(Vec<u8>)),
        "interval" => TypeInfo::force_null(quote!(sqlx::postgres::types::PgInterval)),
        "money" => TypeInfo::force_null(quote!(sqlx::postgres::types::PgMoney)),
        "int4range" => TypeInfo::force_null(quote!(sqlx::postgres::types::PgRange<i32>)),
        "int8range" => TypeInfo::force_null(quote!(sqlx::postgres::types::PgRange<i64>)),
        "timestamptz" => TypeInfo::force_null(quote!(chrono::DateTime<chrono::Utc>)),
        "timestamp" => TypeInfo::force_null(quote!(chrono::NaiveDateTime)),
        "date" => TypeInfo::force_null(quote!(chrono::NaiveDate)),
        "time" => TypeInfo::force_null(quote!(chrono::NaiveTime)),
        "timetz" => TypeInfo::force_null(quote!(sqlx::postgres::types::PgTimeTz)),

        _ => return None,
    };
    Some(ty)
}
