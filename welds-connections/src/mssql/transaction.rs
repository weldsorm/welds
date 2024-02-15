use super::MssqlParam;
use crate::errors::Result;
use crate::Client;
use crate::ExecuteResult;
use crate::Param;
use crate::Row;
use async_trait::async_trait;
use bb8::PooledConnection;
use bb8_tiberius::ConnectionManager;
use tiberius::ToSql;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Open,
    Rolledback,
    Commited,
}

pub(crate) struct MssqlTransaction<'t> {
    conn: PooledConnection<'t, ConnectionManager>,
    state: State,
}

impl<'t> MssqlTransaction<'t> {
    pub async fn new(mut conn: PooledConnection<'t, ConnectionManager>) -> Result<Self> {
        conn.simple_query("BEGIN TRAN").await?;
        Ok(Self {
            conn,
            state: State::Open,
        })
    }

    pub async fn commit(mut self) -> Result<()> {
        self.state = State::Commited;
        self.conn.simple_query("COMMIT").await?;
        Ok(())
    }

    pub async fn rollback(mut self) -> Result<()> {
        self.state = State::Rolledback;
        self.conn.simple_query("ROLLBACK").await?;
        Ok(())
    }
}

impl<'t> MssqlTransaction<'t> {
    /// HACK: returns a mut ref to the trans to run the queries with.
    #[allow(clippy::mut_from_ref)]
    fn as_inner_mut<'a>(&'a self) -> &'a mut PooledConnection<ConnectionManager> {
        // HACK: remove if you can
        // we need a &mut to send to sqlx
        // sqlx need the trans to be mut so it can rollback/commit.
        // we need a mut ref so we can run the queries we send to sqlx
        //
        // This hack should be safe in that the code that calls is NOT allowed to rollback or
        // commit.
        //
        // rollback and commit should only be allowed using the two pub methods
        let conn: &PooledConnection<'t, ConnectionManager> = &self.conn;
        let ptr: *const PooledConnection<ConnectionManager> = conn;
        unsafe {
            let ptr_mut = ptr as *mut PooledConnection<ConnectionManager>;
            &mut *ptr_mut
        }
    }
}

#[async_trait]
impl<'t> Client for MssqlTransaction<'t> {
    async fn execute(&self, sql: &str, params: &[&(dyn Param + Sync)]) -> Result<ExecuteResult> {
        assert_eq!(self.state, State::Open);
        let conn = self.as_inner_mut();
        let mut args: Vec<&dyn ToSql> = Vec::new();
        for &p in params {
            args = MssqlParam::add_param(p, args);
        }
        log::debug!("MSSQL_TRANS_EXEC: {}", sql);
        let r = conn.execute(sql, &args).await?;
        Ok(ExecuteResult {
            rows_affected: r.rows_affected().iter().sum(),
        })
    }

    async fn fetch_rows(&self, sql: &str, params: &[&(dyn Param + Sync)]) -> Result<Vec<Row>> {
        assert_eq!(self.state, State::Open);
        let conn = self.as_inner_mut();
        let mut args: Vec<&dyn ToSql> = Vec::new();
        for &p in params {
            args = MssqlParam::add_param(p, args);
        }
        log::debug!("MSSQL_TRANS_QUERY: {}", sql);
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

impl<'t> Drop for MssqlTransaction<'t> {
    fn drop(&mut self) {
        if self.state != State::Open {
            return;
        }

        // Last resort, Make sure the transaction is rolled back if just dropped
        futures::executor::block_on(async {
            log::warn!("WARNING: transaction was dropped without a commit or rollback. auto-rollback of transaction occurred",);
            self.conn.simple_query("ROLLBACK").await.unwrap();
        })
    }
}
