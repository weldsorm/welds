use super::super::MssqlParam;
use super::ConnectionStatus;
use super::TiberiusConn;
use super::{Client, Param};
use crate::errors::Result;
use crate::row::Row;
use crate::ExecuteResult;
use async_mutex::Mutex as AsyncMutex;
use async_trait::async_trait;
use std::sync::mpsc::Sender;
use tiberius::ToSql;

pub struct PooledConnection {
    pub(crate) status: ConnectionStatus,
    // NOTE: this is an option so it can be taken when dropped
    pub(super) tiberius_conn: AsyncMutex<Option<TiberiusConn>>,
    pub(super) conn_return: Sender<(TiberiusConn, ConnectionStatus)>,
}

impl PooledConnection {
    pub(crate) async fn simple_query(&self, query: &str) -> Result<()> {
        let mut guard = self.tiberius_conn.lock().await;
        let conn: &mut TiberiusConn = guard.as_mut().unwrap();
        conn.simple_query(query).await?;
        Ok(())
    }

    /// returns the number of open transactions for this connection
    pub(crate) async fn transaction_count(&self) -> Result<i32> {
        let mut guard = self.tiberius_conn.lock().await;
        let conn: &mut TiberiusConn = guard.as_mut().unwrap();
        let sql = "SELECT @@TRANCOUNT AS OpenTransactions";
        let set = conn.simple_query(sql).await?;
        let row = set.into_row().await?.unwrap();
        Ok(row.get("OpenTransactions").unwrap())
    }
}

impl Drop for PooledConnection {
    fn drop(&mut self) {
        // take the conn from ourself
        let guard = self.tiberius_conn.get_mut();
        let mut conn: Option<TiberiusConn> = None;
        std::mem::swap(&mut conn, guard);
        let conn = conn.unwrap();
        self.conn_return
            .send((conn, self.status.clone()))
            .expect("Unable to return connection to pool");
    }
}

#[async_trait]
impl Client for PooledConnection {
    async fn execute(&self, sql: &str, params: &[&(dyn Param + Sync)]) -> Result<ExecuteResult> {
        log::trace!("MSSQL EXECUTE: {}", sql);
        let mut guard = self.tiberius_conn.lock().await;
        let conn: &mut TiberiusConn = guard.as_mut().unwrap();

        let mut args: Vec<&dyn ToSql> = Vec::new();
        for &p in params {
            args = MssqlParam::add_param(p, args);
        }
        log::debug!("MSSQL_EXEC: {}", sql);
        let r = conn.execute(sql, &args).await;
        let r = crate::trace::db_error(r)?;
        Ok(ExecuteResult {
            rows_affected: r.rows_affected().iter().sum(),
        })
    }

    async fn fetch_rows(&self, sql: &str, params: &[&(dyn Param + Sync)]) -> Result<Vec<Row>> {
        log::trace!("MSSQL FETCH_ROWS: {}", sql);
        let mut guard = self.tiberius_conn.lock().await;
        let conn: &mut TiberiusConn = guard.as_mut().unwrap();

        let mut args: Vec<&dyn ToSql> = Vec::new();
        for &p in params {
            args = MssqlParam::add_param(p, args);
        }
        let stream = conn.query(sql, &args).await;
        let stream = crate::trace::db_error(stream)?;

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
        let mut guard = self.tiberius_conn.lock().await;
        let conn: &mut TiberiusConn = guard.as_mut().unwrap();
        for fetch in args {
            let sql = fetch.sql;
            log::trace!("MSSQL FETCH_MANY: {}", sql);
            let params = fetch.params;
            let mut args: Vec<&dyn ToSql> = Vec::new();
            for &p in params {
                args = MssqlParam::add_param(p, args);
            }
            let stream = conn.query(sql, &args).await;
            let stream = crate::trace::db_error(stream)?;
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
