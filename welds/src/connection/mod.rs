use anyhow::Result;
use async_trait::async_trait;
use sqlx::database::HasArguments;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DbProvider {
    MySql,
    Mssql,
    Postgres,
    Sqlite,
}

#[async_trait(?Send)]
pub trait Connection<DB: sqlx::Database> {
    async fn execute<'a>(
        &'a self,
        sql: &'a str,
        args: <DB as HasArguments<'a>>::Arguments,
    ) -> Result<()>;

    /// Returns all the data from the resulting query
    async fn fetch_all<'a, T>(
        &'a self,
        sql: &'a str,
        args: <DB as HasArguments<'a>>::Arguments,
    ) -> Result<Vec<T>>
    where
        T: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row>;

    /// Returns one single row from the resulting query
    async fn fetch_one<'a, T>(
        &'a self,
        sql: &'a str,
        args: <DB as HasArguments<'a>>::Arguments,
    ) -> Result<Option<T>>
    where
        T: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row>;

    /// Returns the un-parsed rows
    async fn fetch_rows<'a>(
        &'a self,
        sql: &'a str,
        args: <DB as HasArguments<'a>>::Arguments,
    ) -> Result<Vec<<DB as sqlx::Database>::Row>>;

    /// Returns the un-parsed rows from all the sql statements given
    async fn fetch_many_rows<'a>(
        &'a self,
        statments: Vec<(&'a str, <DB as HasArguments<'a>>::Arguments)>,
    ) -> Result<Vec<<DB as sqlx::Database>::Row>>;

    /// Returns what type of DB you are connected with
    fn provider(&self) -> DbProvider;
}

/// A connection to a database that all welds functions can use.
///
/// The contents of the pool are wrapped in an Arc.
/// Clone the pool is a lite weight operation.
pub struct Pool<DB: sqlx::Database> {
    inner: sqlx::pool::Pool<DB>,
}

impl<DB> Clone for Pool<DB>
where
    DB: sqlx::Database,
{
    fn clone(&self) -> Self {
        Pool {
            inner: self.inner.clone(),
        }
    }
}

impl<DB: sqlx::Database> Pool<DB> {
    /// Return the inner sqlx connection pool
    pub fn as_sqlx_pool(&self) -> &sqlx::pool::Pool<DB> {
        &self.inner
    }

    /// Return the inner sqlx connection pool
    pub async fn begin<'t>(self) -> Result<Transaction<'t, DB>> {
        self::Transaction::new(self).await
    }
}

mod transaction;
pub use transaction::Transaction;

mod any;
/// Used to handle a connection to an unknown-database
pub use any::AnyPool;

#[cfg(feature = "sqlite")]
mod sqlite;
#[cfg(feature = "sqlite")]
/// Connect to a Sqlite Database
pub use sqlite::connect_sqlite;

#[cfg(feature = "mysql")]
mod mysql;
#[cfg(feature = "mysql")]
/// Connect to a MySql Database
pub use mysql::connect_mysql;

#[cfg(feature = "postgres")]
mod postgres;
#[cfg(feature = "postgres")]
/// Connect to a Postgres Database
pub use postgres::connect_postgres;

#[cfg(feature = "mssql")]
mod mssql;
#[cfg(feature = "mssql")]
/// Connect to a Mssql Database
pub use mssql::connect_mssql;
