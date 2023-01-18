use crate::errors::{Result, WeldsError::NoDatabaseUrl, WeldsError::UnsupportedDatabase};
use std::env;

use sqlx::{MssqlPool, MySqlPool, PgPool, SqlitePool};

pub enum Pool {
    Sqlite(SqlitePool),
    MySql(MySqlPool),
    Mssql(MssqlPool),
    Postgres(PgPool),
}

pub async fn connect() -> Result<Pool> {
    let url = env::var("DATABASE_URL").or(Err(NoDatabaseUrl))?;

    if url.starts_with("postgresql:") {
        let pool = PgPool::connect(&url).await?;
        return Ok(Pool::Postgres(pool));
    }

    if url.starts_with("mysql:") {
        let pool = MySqlPool::connect(&url).await?;
        return Ok(Pool::MySql(pool));
    }

    if url.starts_with("sqlite:") {
        let pool = SqlitePool::connect(&url).await?;
        return Ok(Pool::Sqlite(pool));
    }

    if url.starts_with("mssql:") {
        let pool = MssqlPool::connect(&url).await?;
        return Ok(Pool::Mssql(pool));
    }

    Err(UnsupportedDatabase)
}
