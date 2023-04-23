use super::Pool;
use crate::errors::{WeldsError::NoDatabaseUrl, WeldsError::UnsupportedDatabase};
use anyhow::{anyhow, Result};
use std::env;

#[cfg(feature = "mssql")]
use sqlx::Mssql;
#[cfg(feature = "mysql")]
use sqlx::MySql;
#[cfg(feature = "postgres")]
use sqlx::Postgres;
#[cfg(feature = "sqlite")]
use sqlx::Sqlite;

/// A connection to a database of any Kind
/// This can be used when the underlying database isn't known until runtime
pub enum AnyPool {
    #[cfg(feature = "sqlite")]
    Sqlite(Pool<Sqlite>),
    #[cfg(feature = "mysql")]
    MySql(Pool<MySql>),
    #[cfg(feature = "mssql")]
    Mssql(Pool<Mssql>),
    #[cfg(feature = "postgres")]
    Postgres(Pool<Postgres>),
}

#[cfg(feature = "postgres")]
impl From<Pool<Postgres>> for AnyPool {
    fn from(inner: Pool<Postgres>) -> Self {
        AnyPool::Postgres(inner)
    }
}

#[cfg(feature = "mssql")]
impl From<Pool<Mssql>> for AnyPool {
    fn from(inner: Pool<Mssql>) -> Self {
        AnyPool::Mssql(inner)
    }
}

#[cfg(feature = "mysql")]
impl From<Pool<MySql>> for AnyPool {
    fn from(inner: Pool<MySql>) -> Self {
        AnyPool::MySql(inner)
    }
}

#[cfg(feature = "sqlite")]
impl From<Pool<Sqlite>> for AnyPool {
    fn from(inner: Pool<Sqlite>) -> Self {
        AnyPool::Sqlite(inner)
    }
}

impl AnyPool {
    #[cfg(feature = "sqlite")]
    /// Returns a borrowed Pool if the connection is to a Sqlite database
    /// Otherwise None
    pub fn as_sqlite(&self) -> Option<&Pool<Sqlite>> {
        match self {
            AnyPool::Sqlite(inner) => Some(inner),
            _ => None,
        }
    }

    #[cfg(feature = "mysql")]
    /// Returns a borrowed Pool if the connection is to a MySql database
    /// Otherwise None
    pub fn as_mysql(&self) -> Option<&Pool<MySql>> {
        match self {
            AnyPool::MySql(inner) => Some(inner),
            _ => None,
        }
    }

    #[cfg(feature = "mssql")]
    /// Returns a borrowed Pool if the connection is to a Mssql database
    /// Otherwise None
    pub fn as_mssql(&self) -> Option<&Pool<Mssql>> {
        match self {
            AnyPool::Mssql(inner) => Some(inner),
            _ => None,
        }
    }

    #[cfg(feature = "postgres")]
    /// Returns a borrowed Pool if the connection is to a Postgres database
    /// Otherwise None
    pub fn as_postgres(&self) -> Option<&Pool<Postgres>> {
        match self {
            AnyPool::Postgres(inner) => Some(inner),
            _ => None,
        }
    }

    /// Connect to a database using sqlx. used the ENV DATABASE_URL
    pub async fn connect() -> Result<AnyPool> {
        let url = env::var("DATABASE_URL").or(Err(NoDatabaseUrl))?;
        Self::connect_with_connection_string(&url).await
    }

    /// Connect to a database using sqlx
    pub async fn connect_with_connection_string(url: &str) -> Result<AnyPool> {
        #[cfg(feature = "postgres")]
        if url.starts_with("postgresql:") {
            let pool = sqlx::PgPool::connect(url).await?;
            return Ok(AnyPool::Postgres(pool.into()));
        }

        #[cfg(feature = "postgres")]
        if url.starts_with("postgres:") {
            let pool = sqlx::PgPool::connect(url).await?;
            return Ok(AnyPool::Postgres(pool.into()));
        }

        #[cfg(feature = "mysql")]
        if url.starts_with("mysql:") {
            let pool = sqlx::MySqlPool::connect(url).await?;
            return Ok(AnyPool::MySql(pool.into()));
        }

        #[cfg(feature = "sqlite")]
        if url.starts_with("sqlite:") {
            let pool = sqlx::SqlitePool::connect(url).await?;
            return Ok(AnyPool::Sqlite(pool.into()));
        }

        #[cfg(feature = "mssql")]
        if url.starts_with("mssql:") {
            let pool = sqlx::MssqlPool::connect(url).await?;
            return Ok(AnyPool::Mssql(pool.into()));
        }

        Err(anyhow!(UnsupportedDatabase))
    }
}
