use super::transaction::{TransT, Transaction};
use super::Row;
use super::TransactStart;
use super::{Client, Param};
use crate::errors::Result;
use crate::ExecuteResult;
use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;

use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use tiberius::ToSql;

pub(crate) mod transaction;

pub struct MssqlClient {
    pool: Pool<ConnectionManager>,
}

pub(crate) type DbConn = tiberius::Client<tokio_util::compat::Compat<tokio::net::TcpStream>>;

#[async_trait]
impl TransactStart for MssqlClient {
    async fn begin(&self) -> Result<Transaction> {
        // WARNING: we are taking the connection out of the pool. we must put it back when we are finished

        let conn = self.pool.dedicated_connection().await?;
        let conn = Arc::new(Mutex::new(Some(conn)));
        let (tx, rx) = oneshot::channel();

        let trans = transaction::MssqlTransaction::new(tx, conn.clone()).await?;
        let name = trans.trans_name.clone();
        let inner = TransT::Mssql(trans);

        tokio::spawn(async move {
            let needs_rollback: bool = rx.await.unwrap();
            if needs_rollback {
                let mut mine = None;
                {
                    let mut m = conn.lock().unwrap();
                    let inner: &mut Option<_> = &mut m;
                    std::mem::swap(&mut mine, inner);
                }
                let mut coms = mine.unwrap();
                let sql = format!("ROLLBACK TRANSACTION {}", name);
                let _ = coms.simple_query(sql).await;
            }
        });

        Ok(Transaction::new(inner))
    }
}

pub async fn connect(cs: &str) -> Result<MssqlClient> {
    let mgr = bb8_tiberius::ConnectionManager::build(cs)?;
    let pool = bb8::Pool::builder().max_size(2).build(mgr).await.unwrap();
    Ok(MssqlClient { pool })
}

impl From<Pool<ConnectionManager>> for MssqlClient {
    fn from(pool: Pool<ConnectionManager>) -> MssqlClient {
        MssqlClient { pool }
    }
}

impl MssqlClient {
    /// Returns a reference to the underlying tiberius connection
    /// useful when you want to access the database yourself without welds
    pub fn as_tiberius_pool(&mut self) -> &mut Pool<ConnectionManager> {
        &mut self.pool
    }
}

#[async_trait]
impl Client for MssqlClient {
    async fn execute(&self, sql: &str, params: &[&(dyn Param + Sync)]) -> Result<ExecuteResult> {
        let mut conn = self.pool.get().await?;
        let mut args: Vec<&dyn ToSql> = Vec::new();
        for &p in params {
            args = MssqlParam::add_param(p, args);
        }
        log::debug!("MSSQL_EXEC: {}", sql);
        let r = conn.execute(sql, &args).await?;
        Ok(ExecuteResult {
            rows_affected: r.rows_affected().iter().sum(),
        })
    }

    async fn fetch_rows(&self, sql: &str, params: &[&(dyn Param + Sync)]) -> Result<Vec<Row>> {
        let mut conn = self.pool.get().await?;
        let mut args: Vec<&dyn ToSql> = Vec::new();
        for &p in params {
            args = MssqlParam::add_param(p, args);
        }
        log::debug!("MSSQL_QUERY: {}", sql);
        let stream = conn.query(sql, &args).await?;

        let mssql_rows = stream.into_results().await?;
        let mut all = Vec::default();
        for batch in mssql_rows {
            for r in batch {
                all.push(Row::from(r))
            }
        }
        Ok(all)
    }

    async fn fetch_many<'s, 'args, 't>(
        &self,
        args: &[crate::Fetch<'s, 'args, 't>],
    ) -> Result<Vec<Vec<Row>>> {
        let mut resultset = Vec::default();
        let mut conn = self.pool.get().await?;
        for fetch in args {
            let sql = fetch.sql;
            let params = fetch.params;
            let mut args: Vec<&dyn ToSql> = Vec::new();
            for &p in params {
                args = MssqlParam::add_param(p, args);
            }
            log::debug!("MSSQL_QUERY: {}", sql);
            let stream = conn.query(sql, &args).await?;
            let mssql_rows = stream.into_results().await?;
            let mut all = Vec::default();
            for batch in mssql_rows {
                for r in batch {
                    all.push(Row::from(r))
                }
            }
            resultset.push(all)
        }
        Ok(resultset)
    }

    fn syntax(&self) -> crate::Syntax {
        crate::Syntax::Mssql
    }
}

//impl<T> Param for T where T: MssqlParam {}

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
