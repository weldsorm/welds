use super::pool::ConnectionStatus;
use super::pool::PooledConnection;
use crate::Client;
use crate::ExecuteResult;
use crate::Param;
use crate::Row;
use crate::errors::Error::ClosedTransaction;
use crate::errors::Result;
use async_trait::async_trait;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Open,
    Rolledback,
    Commited,
}

pub(crate) struct MssqlTransaction<'t> {
    conn: PooledConnection,
    state: State,
    _phantom: PhantomData<&'t ()>,
    pub(crate) trans_name: String,
}

impl MssqlTransaction<'_> {
    pub(crate) async fn new(mut conn: PooledConnection) -> Result<Self> {
        // start the transaction
        let trans_name = format!("t_{}", get_trans_count());
        let sql = format!("BEGIN TRANSACTION {}", trans_name);
        conn.simple_query(&sql).await?;
        // mark the connection as needing a rollback
        conn.status = ConnectionStatus::NeedsRollback(trans_name.clone());

        Ok(Self {
            conn,
            state: State::Open,
            _phantom: Default::default(),
            trans_name,
        })
    }

    pub async fn commit(mut self) -> Result<()> {
        log::debug!("MSSQL COMMIT: {}", self.trans_name);
        assert_eq!(self.state, State::Open);
        self.state = State::Commited;
        let sql = format!("COMMIT TRANSACTION {}", self.trans_name);
        self.conn.simple_query(&sql).await?;
        self.conn.status = ConnectionStatus::Clean;
        Ok(())
    }

    pub async fn rollback(mut self) -> Result<()> {
        log::debug!("MSSQL ROLLBACK: {}", self.trans_name);
        if self.state == State::Rolledback {
            return Ok(());
        }
        assert_eq!(self.state, State::Open);
        self.state = State::Rolledback;
        let sql = format!("ROLLBACK TRANSACTION {}", self.trans_name);
        self.conn.simple_query(&sql).await?;
        self.conn.status = ConnectionStatus::Clean;
        Ok(())
    }

    /// MSSQL will auto rollback a transaction on some errors.
    /// If this has happened, mark this transaction as being closed.
    pub(crate) async fn internal_rollback_check(&mut self) -> Result<()> {
        log::debug!("MSSQL ROLLBACK INTERNAL: {}", self.trans_name);
        if self.state == State::Rolledback {
            return Ok(());
        }
        if self.conn.transaction_count().await? == 0 {
            self.state = State::Rolledback;
            self.conn.status = ConnectionStatus::Clean;
        }
        Ok(())
    }
}

#[async_trait]
impl Client for MssqlTransaction<'_> {
    async fn execute(&self, sql: &str, params: &[&(dyn Param + Sync)]) -> Result<ExecuteResult> {
        if self.state != State::Open {
            return Err(ClosedTransaction);
        }
        self.conn.execute(sql, params).await
    }

    async fn fetch_rows(&self, sql: &str, params: &[&(dyn Param + Sync)]) -> Result<Vec<Row>> {
        if self.state != State::Open {
            return Err(ClosedTransaction);
        }
        self.conn.fetch_rows(sql, params).await
    }

    async fn fetch_many<'s, 'args, 'i>(
        &self,
        fetches: &[crate::Fetch<'s, 'args, 'i>],
    ) -> Result<Vec<Vec<Row>>> {
        if self.state != State::Open {
            return Err(ClosedTransaction);
        }
        self.conn.fetch_many(fetches).await
    }

    fn syntax(&self) -> crate::Syntax {
        crate::Syntax::Mssql
    }
}

use std::sync::atomic::{AtomicUsize, Ordering};

static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

fn get_trans_count() -> usize {
    CALL_COUNT.fetch_add(1, Ordering::SeqCst) + 1
}
