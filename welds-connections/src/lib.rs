pub use crate::errors::Error;
use crate::errors::Result;
use async_trait::async_trait;
pub use row::Row;
pub use transaction::Transaction;
pub mod errors;
pub mod row;
pub mod transaction;
pub mod trace;
pub mod any;
#[cfg(feature = "mssql")]
pub mod mssql;
#[cfg(feature = "mysql")]
pub mod mysql;
#[cfg(feature = "noop")]
pub mod noop;
#[cfg(feature = "postgres")]
pub mod postgres;
#[cfg(feature = "sqlite")]
pub mod sqlite;

pub struct Fetch<'s, 'args, 't> {
    pub sql: &'s str,
    pub params: &'args [&'t (dyn Param + Sync)],
}

#[async_trait]
/// The common trait for database connections and transactions.
pub trait Client: Sync + Send {
    /// Execute a sql command. returns the number of rows that were affected
    async fn execute(&self, sql: &str, params: &[&(dyn Param + Sync)]) -> Result<ExecuteResult>;

    /// Runs SQL and returns a collection of rows from the database.
    async fn fetch_rows(&self, sql: &str, params: &[&(dyn Param + Sync)]) -> Result<Vec<Row>>;

    /// Run several `fetch_rows` command on the same connection in the connection pool
    async fn fetch_many<'s, 'args, 't>(
        &self,
        args: &[Fetch<'s, 'args, 't>],
    ) -> Result<Vec<Vec<Row>>>;

    // Returns what syntax (dialect) of SQL the backend is expecting
    fn syntax(&self) -> Syntax;
}

/// Used the ENV DATABASE_URL
/// builds a connection with whatever is in it.
pub async fn connect_from_env() -> Result<any::AnyClient> {
    let url = std::env::var("DATABASE_URL").or(Err(Error::InvalidDatabaseUrl))?;
    connect(&url).await
}

/// Returns a connection pool (Client/TransactStart) for the given connection string.
///
/// To use, make sure your database feature is enabled
///
/// connection string formats:
/// SQLX Connection String (postgres, mysql, sqlite)
/// ADO Connection String (mssql)
pub async fn connect(cs: impl Into<String>) -> Result<any::AnyClient> {
    let cs: String = cs.into();
    #[cfg(feature = "postgres")]
    if cs.starts_with("postgresql:") {
        log::debug!("Welds connecting to Postgres");
        let client = postgres::connect(&cs).await?;
        return Ok(any::AnyClient::Postgres(client));
    }
    #[cfg(feature = "postgres")]
    if cs.starts_with("postgres:") {
        log::debug!("Welds connecting to Postgres");
        let client = postgres::connect(&cs).await?;
        return Ok(any::AnyClient::Postgres(client));
    }
    #[cfg(feature = "mysql")]
    if cs.starts_with("mysql:") {
        log::debug!("Welds connecting to MySql");
        let client = mysql::connect(&cs).await?;
        return Ok(any::AnyClient::Mysql(client));
    }
    #[cfg(feature = "sqlite")]
    if cs.starts_with("sqlite:") {
        log::debug!("Welds connecting to Sqlite");
        let client = sqlite::connect(&cs).await?;
        return Ok(any::AnyClient::Sqlite(client));
    }
    #[cfg(feature = "mssql")]
    if !cs.is_empty() {
        log::debug!("Welds connecting to MSSQL");
        let client = mssql::connect(&cs).await?;
        return Ok(any::AnyClient::Mssql(client));
    }
    Err(errors::Error::InvalidDatabaseUrl)
}

#[async_trait]
/// Implementers of this trait can crate a transaction.
/// If you want to create a transaction off of a Client,
/// make sure you `use welds::TransactStart`
pub trait TransactStart {
    async fn begin<'t>(&'t self) -> Result<Transaction<'t>>;
}

// This code is scripted out cuz writing it for all the features to be to much
mod params;
pub use params::Param;

pub struct ExecuteResult {
    pub(crate) rows_affected: u64,
}

impl ExecuteResult {
    pub fn new(rows_affected: u64) -> Self {
        Self { rows_affected }
    }

    pub fn rows_affected(&self) -> u64 {
        self.rows_affected
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Syntax {
    Mysql,
    Postgres,
    Sqlite,
    Mssql,
}
