pub use crate::errors::Error;
use crate::errors::Result;
use async_trait::async_trait;
pub use row::Row;
pub use transaction::Transaction;
mod errors;
pub mod row;
pub mod transaction;

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

#[async_trait]
/// The common trait for database connections and transactions.
pub trait Client {
    /// Execute a sql command. returns the number of rows that were affected
    async fn execute(&self, sql: &str, params: &[&(dyn Param + Sync)]) -> Result<ExecuteResult>;
    /// Runs SQL and returns a collection of rows from the database.
    async fn fetch_rows(&self, sql: &str, params: &[&(dyn Param + Sync)]) -> Result<Vec<Row>>;
    // Returns what syntax (dialect) of SQL the backend is expecting
    fn syntax(&self) -> Syntax;
}

#[async_trait]
/// Implementers of this trait can crate a transaction.
/// If you want to create a transaction off of a Client,
/// make sure you `use welds::TransactStart`
pub trait TransactStart {
    async fn begin(&self) -> Result<Transaction>;
}

// This code is scripted out cuz writing it for all the features to be to much
mod params;
pub use params::Param;

pub struct ExecuteResult {
    pub(crate) rows_affected: u64,
}

impl ExecuteResult {
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
