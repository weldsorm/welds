use super::Client;
use super::ExecuteResult;
use super::Param;
use super::Row;
use super::Syntax;
use crate::errors::Result;
use crate::Fetch;
use crate::{TransactStart, Transaction};
use async_trait::async_trait;

/// This is a wrapper around a connection that could be Any underlying database
/// only for the connection type features that have been enabled
#[derive(Clone)]
pub enum AnyClient {
    #[cfg(feature = "sqlite")]
    Sqlite(crate::sqlite::SqliteClient),
    #[cfg(feature = "postgres")]
    Postgres(crate::postgres::PostgresClient),
    #[cfg(feature = "mysql")]
    Mysql(crate::mysql::MysqlClient),
    #[cfg(feature = "mssql")]
    Mssql(crate::mssql::MssqlClient),
    #[cfg(feature = "noop")]
    Noop(crate::noop::NoopClient),
}

#[async_trait]
impl Client for AnyClient {
    /// Execute a sql command. returns the number of rows that were affected
    async fn execute(&self, sql: &str, params: &[&(dyn Param + Sync)]) -> Result<ExecuteResult> {
        match self {
            #[cfg(feature = "sqlite")]
            AnyClient::Sqlite(c) => c.execute(sql, params).await,
            #[cfg(feature = "postgres")]
            AnyClient::Postgres(c) => c.execute(sql, params).await,
            #[cfg(feature = "mysql")]
            AnyClient::Mysql(c) => c.execute(sql, params).await,
            #[cfg(feature = "mssql")]
            AnyClient::Mssql(c) => c.execute(sql, params).await,
            #[cfg(feature = "noop")]
            AnyClient::Noop(c) => c.execute(sql, params).await,
        }
    }

    /// Runs SQL and returns a collection of rows from the database.
    async fn fetch_rows(&self, sql: &str, params: &[&(dyn Param + Sync)]) -> Result<Vec<Row>> {
        match self {
            #[cfg(feature = "sqlite")]
            AnyClient::Sqlite(c) => c.fetch_rows(sql, params).await,
            #[cfg(feature = "postgres")]
            AnyClient::Postgres(c) => c.fetch_rows(sql, params).await,
            #[cfg(feature = "mysql")]
            AnyClient::Mysql(c) => c.fetch_rows(sql, params).await,
            #[cfg(feature = "mssql")]
            AnyClient::Mssql(c) => c.fetch_rows(sql, params).await,
            #[cfg(feature = "noop")]
            AnyClient::Noop(c) => c.fetch_rows(sql, params).await,
        }
    }

    /// Run several `fetch_rows` command on the same connection in the connection pool
    async fn fetch_many<'s, 'args, 't>(
        &self,
        args: &[Fetch<'s, 'args, 't>],
    ) -> Result<Vec<Vec<Row>>> {
        match self {
            #[cfg(feature = "sqlite")]
            AnyClient::Sqlite(c) => c.fetch_many(args).await,
            #[cfg(feature = "postgres")]
            AnyClient::Postgres(c) => c.fetch_many(args).await,
            #[cfg(feature = "mysql")]
            AnyClient::Mysql(c) => c.fetch_many(args).await,
            #[cfg(feature = "mssql")]
            AnyClient::Mssql(c) => c.fetch_many(args).await,
            #[cfg(feature = "noop")]
            AnyClient::Noop(c) => c.fetch_many(args).await,
        }
    }

    // Returns what syntax (dialect) of SQL the backend is expecting
    fn syntax(&self) -> Syntax {
        match self {
            #[cfg(feature = "sqlite")]
            AnyClient::Sqlite(c) => c.syntax(),
            #[cfg(feature = "postgres")]
            AnyClient::Postgres(c) => c.syntax(),
            #[cfg(feature = "mysql")]
            AnyClient::Mysql(c) => c.syntax(),
            #[cfg(feature = "mssql")]
            AnyClient::Mssql(c) => c.syntax(),
            #[cfg(feature = "noop")]
            AnyClient::Noop(c) => c.syntax(),
        }
    }
}

#[async_trait]
impl TransactStart for AnyClient {
    async fn begin<'t>(&'t self) -> Result<Transaction<'t>> {
        match self {
            #[cfg(feature = "sqlite")]
            AnyClient::Sqlite(c) => c.begin().await,
            #[cfg(feature = "postgres")]
            AnyClient::Postgres(c) => c.begin().await,
            #[cfg(feature = "mysql")]
            AnyClient::Mysql(c) => c.begin().await,
            #[cfg(feature = "mssql")]
            AnyClient::Mssql(c) => c.begin().await,
            #[cfg(feature = "noop")]
            AnyClient::Noop(_) => panic!("transaction not supporting in test mode"),
        }
    }
}

impl AsRef<AnyClient> for AnyClient {
    fn as_ref(&self) -> &AnyClient {
        self
    }
}
