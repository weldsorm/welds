use super::transaction::{TransT, Transaction};
use super::Row;
use super::TransactStart;
use super::{Client, Param};
use crate::errors::Result;
use crate::ExecuteResult;
use async_trait::async_trait;
use std::sync::Arc;

use tiberius::ToSql;
mod pool;
use pool::Pool;

pub(crate) mod transaction;

#[derive(Clone)]
pub struct MssqlClient {
    pool: Arc<Pool>,
}

#[async_trait]
impl TransactStart for MssqlClient {
    async fn begin<'t>(&'t self) -> Result<Transaction<'t>> {
        let conn = self.pool.get().await?;
        log::debug!("TransactStart: building transaction");
        let trans = transaction::MssqlTransaction::new(conn).await?;
        let inner = TransT::Mssql(trans);
        Ok(Transaction::new(inner))
    }
}

pub async fn connect(cs: &str) -> Result<MssqlClient> {
    let mgr = bb8_tiberius::ConnectionManager::build(cs)?;
    let pool = Pool::new(mgr);
    Ok(MssqlClient { pool })
}

impl MssqlClient {
    /// Returns a reference to the underlying tiberius connection
    /// useful when you want to access the database yourself without welds
    pub fn as_tiberius_pool(&self) -> Arc<Pool> {
        self.pool.clone()
    }
}

#[async_trait]
impl Client for MssqlClient {
    async fn execute(&self, sql: &str, params: &[&(dyn Param + Sync)]) -> Result<ExecuteResult> {
        let conn = self.pool.get().await?;
        conn.execute(sql, params).await
    }

    async fn fetch_rows(&self, sql: &str, params: &[&(dyn Param + Sync)]) -> Result<Vec<Row>> {
        let conn = self.pool.get().await?;
        conn.fetch_rows(sql, params).await
    }

    async fn fetch_many<'s, 'args, 't>(
        &self,
        args: &[crate::Fetch<'s, 'args, 't>],
    ) -> Result<Vec<Vec<Row>>> {
        let conn = self.pool.get().await?;
        conn.fetch_many(args).await
    }

    fn syntax(&self) -> crate::Syntax {
        crate::Syntax::Mssql
    }
}

pub trait MssqlParam {
    fn add_param<'a>(&'a self, args: Vec<&'a dyn ToSql>) -> Vec<&'a dyn ToSql>;
}

impl<T> MssqlParam for T
where
    T: 'static + ToSql,
{
    fn add_param<'a>(&'a self, mut args: Vec<&'a dyn ToSql>) -> Vec<&'a dyn ToSql> {
        args.push(self);
        args
    }
}
