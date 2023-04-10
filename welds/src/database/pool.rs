use crate::errors::{Result, WeldsError::NoDatabaseUrl, WeldsError::UnsupportedDatabase};
use std::env;

#[cfg(feature = "mssql")]
use sqlx::MssqlPool;
#[cfg(feature = "mysql")]
use sqlx::MySqlPool;
#[cfg(feature = "postgres")]
use sqlx::PgPool;
#[cfg(feature = "sqlite")]
use sqlx::SqlitePool;

#[derive(Debug)]
pub enum Pool {
    #[cfg(feature = "sqlite")]
    Sqlite(SqlitePool),
    #[cfg(feature = "mysql")]
    MySql(MySqlPool),
    #[cfg(feature = "mssql")]
    Mssql(MssqlPool),
    #[cfg(feature = "postgres")]
    Postgres(PgPool),
}

#[cfg(feature = "postgres")]
impl From<PgPool> for Pool {
    fn from(inner: PgPool) -> Self {
        Pool::Postgres(inner)
    }
}

#[cfg(feature = "mssql")]
impl From<MssqlPool> for Pool {
    fn from(inner: MssqlPool) -> Self {
        Pool::Mssql(inner)
    }
}

#[cfg(feature = "sqlite")]
impl From<SqlitePool> for Pool {
    fn from(inner: SqlitePool) -> Self {
        Pool::Sqlite(inner)
    }
}

#[cfg(feature = "mysql")]
impl From<MySqlPool> for Pool {
    fn from(inner: MySqlPool) -> Self {
        Pool::MySql(inner)
    }
}

impl Pool {
    #[cfg(feature = "sqlite")]
    pub fn as_sqlite<'a>(&'a self) -> Option<&'a SqlitePool> {
        match self {
            Pool::Sqlite(inner) => Some(inner),
            _ => None,
        }
    }

    #[cfg(feature = "mysql")]
    pub fn as_mysql<'a>(&'a self) -> Option<&'a MySqlPool> {
        match self {
            Pool::MySql(inner) => Some(inner),
            _ => None,
        }
    }

    #[cfg(feature = "mssql")]
    pub fn as_mssql<'a>(&'a self) -> Option<&'a MssqlPool> {
        match self {
            Pool::Mssql(inner) => Some(inner),
            _ => None,
        }
    }

    #[cfg(feature = "postgres")]
    pub fn as_postgres<'a>(&'a self) -> Option<&'a PgPool> {
        match self {
            Pool::Postgres(inner) => Some(inner),
            _ => None,
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
    #[cfg(feature = "postgres")]
    if url.starts_with("postgresql:") {
        let pool = PgPool::connect(&url).await?;
        return Ok(Pool::Postgres(pool));
    }

    #[cfg(feature = "postgres")]
    if url.starts_with("postgres:") {
        let pool = PgPool::connect(&url).await?;
        return Ok(Pool::Postgres(pool));
    }

    #[cfg(feature = "mysql")]
    if url.starts_with("mysql:") {
        let pool = MySqlPool::connect(&url).await?;
        return Ok(Pool::MySql(pool));
    }

    #[cfg(feature = "sqlite")]
    if url.starts_with("sqlite:") {
        let pool = SqlitePool::connect(&url).await?;
        return Ok(Pool::Sqlite(pool));
    }

    #[cfg(feature = "mssql")]
    if url.starts_with("mssql:") {
        let pool = MssqlPool::connect(&url).await?;
        return Ok(Pool::Mssql(pool));
    }

    Err(UnsupportedDatabase)
}
