use super::MssqlParam;
use crate::errors::Result;
use crate::Client;
use crate::ExecuteResult;
use crate::Param;
use crate::Row;
use async_trait::async_trait;
use bb8::PooledConnection;
use bb8_tiberius::ConnectionManager;
use std::sync::Mutex;
use tiberius::ToSql;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Open,
    Rolledback,
    Commited,
}

pub(crate) struct MssqlTransaction<'t> {
    conn: Mutex<Option<PooledConnection<'t, ConnectionManager>>>,
    state: State,
}

impl<'t> MssqlTransaction<'t> {
    pub async fn new(mut conn: PooledConnection<'t, ConnectionManager>) -> Result<Self> {
        conn.simple_query("BEGIN TRAN").await?;
        Ok(Self {
            conn: Mutex::new(Some(conn)),
            state: State::Open,
        })
    }

    pub async fn commit(mut self) -> Result<()> {
        self.state = State::Commited;
        let mut conn = self.take_conn();
        conn.simple_query("COMMIT").await?;
        self.return_conn(conn);
        Ok(())
    }

    pub async fn rollback(mut self) -> Result<()> {
        self.state = State::Rolledback;
        let mut conn = self.take_conn();
        conn.simple_query("ROLLBACK").await?;
        self.return_conn(conn);
        Ok(())
    }
}

impl<'t> MssqlTransaction<'t> {
    // HACK - CODE SMELL:
    // we need a &mut conn for the connection pool
    // this (take_conn/return_conn) acts like a CellRef
    // It will panic if you try to the conn more one at at time
    //
    fn take_conn(&self) -> PooledConnection<'t, ConnectionManager> {
        let mut placeholder = None;
        let mut m = self.conn.lock().unwrap();
        let inner: &mut Option<PooledConnection<ConnectionManager>> = &mut m;
        // Panic if the conn is already taken
        assert!(inner.is_some(), "Pool was already taken");
        std::mem::swap(&mut placeholder, inner);
        placeholder.unwrap()
    }
    fn return_conn(&self, conn: PooledConnection<'t, ConnectionManager>) {
        let mut placeholder = Some(conn);
        let mut m = self.conn.lock().unwrap();
        let inner: &mut Option<PooledConnection<ConnectionManager>> = &mut m;
        // Panic if we already have a the conn
        assert!(inner.is_none(), "Overriding existing pool");
        std::mem::swap(&mut placeholder, inner);
    }
}

#[async_trait]
impl<'t> Client for MssqlTransaction<'t> {
    async fn execute(&self, sql: &str, params: &[&(dyn Param + Sync)]) -> Result<ExecuteResult> {
        assert_eq!(self.state, State::Open);
        let mut conn = self.take_conn();
        let mut args: Vec<&dyn ToSql> = Vec::new();
        for &p in params {
            args = MssqlParam::add_param(p, args);
        }
        log::debug!("MSSQL_TRANS_EXEC: {}", sql);
        let r = conn.execute(sql, &args).await;
        self.return_conn(conn);
        let r = r?;

        Ok(ExecuteResult {
            rows_affected: r.rows_affected().iter().sum(),
        })
    }

    async fn fetch_rows(&self, sql: &str, params: &[&(dyn Param + Sync)]) -> Result<Vec<Row>> {
        assert_eq!(self.state, State::Open);
        let mut conn = self.take_conn();
        let results = fetch_rows_inner(&mut conn, sql, params).await;
        self.return_conn(conn);
        let rows = results?;
        Ok(rows)
    }

    async fn fetch_many<'s, 'args, 'i>(
        &self,
        fetches: &[crate::Fetch<'s, 'args, 'i>],
    ) -> Result<Vec<Vec<Row>>> {
        assert_eq!(self.state, State::Open);
        let mut conn = self.take_conn();
        let mut results = Vec::default();
        for fetch in fetches {
            let sql = fetch.sql;
            let params = fetch.params;
            let r = fetch_rows_inner(&mut conn, sql, params).await;
            let is_err = r.is_err();
            results.push(r);
            if is_err {
                break;
            }
        }
        self.return_conn(conn);
        results.drain(..).collect()
    }

    fn syntax(&self) -> crate::Syntax {
        crate::Syntax::Mssql
    }
}

async fn fetch_rows_inner<'t>(
    conn: &mut PooledConnection<'t, ConnectionManager>,
    sql: &str,
    params: &[&(dyn Param + Sync)],
) -> Result<Vec<Row>> {
    let mut args: Vec<&dyn ToSql> = Vec::new();
    for &p in params {
        args = MssqlParam::add_param(p, args);
    }
    log::debug!("MSSQL_TRANS_QUERY: {}", sql);

    let stream = conn.query(sql, &args).await;
    let stream = stream?;

    let mssql_rows = stream.into_results().await?;
    let mut all = Vec::default();
    for batch in mssql_rows {
        for r in batch {
            all.push(Row::from(r))
        }
    }
    Ok(all)
}

impl<'t> Drop for MssqlTransaction<'t> {
    fn drop(&mut self) {
        if self.state != State::Open {
            return;
        }

        // Last resort, Make sure the transaction is rolled back if just dropped
        futures::executor::block_on(async {
            log::warn!("WARNING: transaction was dropped without a commit or rollback. auto-rollback of transaction occurred",);
            let mut conn = self.take_conn();
            conn.simple_query("ROLLBACK").await.unwrap();
        })
    }
}
