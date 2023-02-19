use crate::errors::{Result, WeldsError::NoDatabaseUrl, WeldsError::UnsupportedDatabase};
use crate::query::GenericQueryBuilder;
use std::env;

use sqlx::{MssqlPool, MySqlPool, PgPool, SqlitePool};

pub enum Pool {
    Sqlite(SqlitePool),
    MySql(MySqlPool),
    Mssql(MssqlPool),
    Postgres(PgPool),
}

impl From<PgPool> for Pool {
    fn from(inner: PgPool) -> Self {
        Pool::Postgres(inner)
    }
}

impl From<MssqlPool> for Pool {
    fn from(inner: MssqlPool) -> Self {
        Pool::Mssql(inner)
    }
}

impl From<SqlitePool> for Pool {
    fn from(inner: SqlitePool) -> Self {
        Pool::Sqlite(inner)
    }
}

impl From<MySqlPool> for Pool {
    fn from(inner: MySqlPool) -> Self {
        Pool::MySql(inner)
    }
}

impl Pool {
    pub fn as_sqlite<'a>(&'a self) -> Option<&'a SqlitePool> {
        match self {
            Pool::Sqlite(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn as_mysql<'a>(&'a self) -> Option<&'a MySqlPool> {
        match self {
            Pool::MySql(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn as_mssql<'a>(&'a self) -> Option<&'a MssqlPool> {
        match self {
            Pool::Mssql(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn as_postgres<'a>(&'a self) -> Option<&'a PgPool> {
        match self {
            Pool::Postgres(inner) => Some(inner),
            _ => None,
        }
    }

    /// Creates a query builder for this connection
    pub fn querybuilder<'args>(&self) -> GenericQueryBuilder<'args> {
        type QB<'args> = GenericQueryBuilder<'args>;
        type QB1<'args> = sqlx::QueryBuilder<'args, sqlx::Sqlite>;
        type QB2<'args> = sqlx::QueryBuilder<'args, sqlx::Mssql>;
        type QB3<'args> = sqlx::QueryBuilder<'args, sqlx::MySql>;
        type QB4<'args> = sqlx::QueryBuilder<'args, sqlx::Postgres>;
        match self {
            Pool::Sqlite(_) => QB::Sqlite(QB1::new("")),
            Pool::Mssql(_) => QB::Mssql(QB2::new("")),
            Pool::MySql(_) => QB::MySql(QB3::new("")),
            Pool::Postgres(_) => QB::Postgres(QB4::new("")),
        }
    }
}

/// Connect to a database using sqlx. used the ENV DATABASE_URL
pub async fn connect() -> Result<Pool> {
    let url = env::var("DATABASE_URL").or(Err(NoDatabaseUrl))?;
    connect_with_connection_string(&url).await
}

/// Connect to a database using sqlx
pub async fn connect_with_connection_string(url: &str) -> Result<Pool> {
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
