use super::transaction::{TransT, Transaction};
use super::Row;
use super::TransactStart;
use super::{Client, Param};
use crate::errors::Result;
use crate::ExecuteResult;
use async_trait::async_trait;

use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use tiberius::ToSql;

pub(crate) mod transaction;

pub struct MssqlClient {
    pool: Pool<ConnectionManager>,
}

#[async_trait]
impl TransactStart for MssqlClient {
    async fn begin(&self) -> Result<Transaction> {
        let conn = self.pool.get().await?;
        let trans = transaction::MssqlTransaction::new(conn).await?;
        let inner = TransT::Mssql(trans);
        Ok(Transaction::new(inner))
    }
}

pub async fn get_conn(cs: &str) -> Result<MssqlClient> {
    let mgr = bb8_tiberius::ConnectionManager::build(cs)?;
    let pool = bb8::Pool::builder().max_size(2).build(mgr).await.unwrap();
    Ok(MssqlClient { pool })
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
