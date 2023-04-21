use anyhow::Result;
use async_trait::async_trait;
use sqlx::database::HasArguments;
use std::cell::RefCell;

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
}

#[derive(Clone)]
pub struct Pool<DB: sqlx::Database> {
    inner: sqlx::pool::Pool<DB>,
}

impl<DB: sqlx::Database> Pool<DB> {
    /// Return the inner sqlx connection pool
    pub fn as_sqlx_pool(&self) -> &sqlx::pool::Pool<DB> {
        &self.inner
    }

    /// Return the inner sqlx connection pool
    pub async fn begin(&self) -> Result<Transaction<DB>> {
        let inner = self.inner.begin().await?;
        let inner = RefCell::new(inner);
        Ok(self::Transaction { inner })
    }
}

pub struct Transaction<'trans, DB: sqlx::Database> {
    inner: RefCell<sqlx::Transaction<'trans, DB>>,
}

impl<'trans, DB: sqlx::Database> Transaction<'trans, DB> {
    /// Rollback the transaction
    pub async fn rollback(self) -> Result<()> {
        let inner = self.inner.into_inner();
        inner.rollback().await?;
        Ok(())
    }
    /// Rollback the transaction
    pub async fn commit(self) -> Result<()> {
        let inner = self.inner.into_inner();
        inner.commit().await?;
        Ok(())
    }
}

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
