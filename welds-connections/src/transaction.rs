use super::Row;
use super::{Client, Param};
use crate::errors::Result;
use crate::{ExecuteResult, Syntax};
use async_trait::async_trait;
use std::sync::Mutex;

#[cfg(feature = "mssql")]
use crate::mssql::transaction::MssqlTransaction;

pub struct Transaction<'t> {
    inner: Mutex<TransT<'t>>,
    syntax: crate::Syntax,
}

impl<'t> Transaction<'t> {
    pub(crate) fn new(inner: TransT<'t>) -> Self {
        let syntax = match &inner {
            #[cfg(feature = "sqlite")]
            TransT::Sqlite(_) => Syntax::Sqlite,
            #[cfg(feature = "mssql")]
            TransT::Mssql(_) => Syntax::Mssql,
            #[cfg(feature = "postgres")]
            TransT::Postgres(_) => Syntax::Postgres,
            #[cfg(feature = "mysql")]
            TransT::Mysql(_) => Syntax::Mysql,
        };

        Self {
            syntax,
            inner: Mutex::new(inner),
        }
    }

    pub async fn rollback(self) -> Result<()> {
        let inner = self.inner.into_inner().unwrap();
        inner.rollback().await?;
        Ok(())
    }
    pub async fn commit(self) -> Result<()> {
        let inner = self.inner.into_inner().unwrap();
        inner.commit().await?;
        Ok(())
    }
}

pub(crate) enum TransT<'t> {
    #[cfg(feature = "sqlite")]
    Sqlite(sqlx::Transaction<'t, sqlx::Sqlite>),
    #[cfg(feature = "postgres")]
    Postgres(sqlx::Transaction<'t, sqlx::Postgres>),
    #[cfg(feature = "mysql")]
    Mysql(sqlx::Transaction<'t, sqlx::MySql>),
    #[cfg(feature = "mssql")]
    Mssql(MssqlTransaction<'t>),
}

impl<'t> TransT<'t> {
    async fn rollback(self) -> Result<()> {
        match self {
            #[cfg(feature = "sqlite")]
            TransT::Sqlite(t) => t.rollback().await?,
            #[cfg(feature = "mssql")]
            TransT::Mssql(t) => t.rollback().await?,
            #[cfg(feature = "postgres")]
            TransT::Postgres(t) => t.rollback().await?,
            #[cfg(feature = "mysql")]
            TransT::Mysql(t) => t.rollback().await?,
        }
        Ok(())
    }
    async fn commit(self) -> Result<()> {
        match self {
            #[cfg(feature = "sqlite")]
            TransT::Sqlite(t) => t.commit().await?,
            #[cfg(feature = "mssql")]
            TransT::Mssql(t) => t.commit().await?,
            #[cfg(feature = "postgres")]
            TransT::Postgres(t) => t.commit().await?,
            #[cfg(feature = "mysql")]
            TransT::Mysql(t) => t.commit().await?,
        }
        Ok(())
    }
}

impl<'t> Transaction<'t> {
    /// HACK: returns a mut ref to the trans to run the queries with.
    #[allow(clippy::mut_from_ref)]
    fn as_inner_mut<'a>(&'a self) -> &'a mut TransT<'a> {
        // HACK: remove if you can
        // we need a &mut to send to sqlx
        // sqlx need the trans to be mut so it can rollback/commit.
        // we need a mut ref so we can run the queries we send to sqlx
        //
        // This hack should be safe in that the code that calls is NOT allowed to rollback or
        // commit.
        //
        // rollback and commit should only be allowed using the two pub methods
        let lock = self.inner.lock().unwrap();
        let b: &TransT = &lock;
        let ptr: *const TransT = b;
        unsafe {
            let ptr_mut = ptr as *mut TransT;
            &mut *ptr_mut
        }
    }
}

#[cfg(feature = "mysql")]
use super::mysql::MysqlParam;
#[cfg(feature = "postgres")]
use super::postgres::PostgresParam;
#[cfg(feature = "sqlite")]
use super::sqlite::SqliteParam;

#[async_trait]
impl<'t> Client for Transaction<'t> {
    fn syntax(&self) -> crate::Syntax {
        self.syntax
    }

    async fn execute(&self, sql: &str, params: &[&(dyn Param + Sync)]) -> Result<ExecuteResult> {
        let mut inner = self.as_inner_mut();
        match &mut inner {
            #[cfg(feature = "sqlite")]
            TransT::Sqlite(t) => {
                let x: &mut <sqlx::Sqlite as sqlx::Database>::Connection = t;
                let mut query = sqlx::query::<sqlx::Sqlite>(sql);
                for param in params {
                    query = SqliteParam::add_param(*param, query)
                }
                let t = query.execute(x).await?;
                Ok(ExecuteResult {
                    rows_affected: t.rows_affected(),
                })
            }

            #[cfg(feature = "postgres")]
            TransT::Postgres(t) => {
                let x: &mut <sqlx::Postgres as sqlx::Database>::Connection = t;
                let mut query = sqlx::query::<sqlx::Postgres>(sql);
                for param in params {
                    query = PostgresParam::add_param(*param, query)
                }
                let t = query.execute(x).await?;
                Ok(ExecuteResult {
                    rows_affected: t.rows_affected(),
                })
            }

            #[cfg(feature = "mysql")]
            TransT::Mysql(t) => {
                let x: &mut <sqlx::MySql as sqlx::Database>::Connection = t;
                let mut query = sqlx::query::<sqlx::MySql>(sql);
                for param in params {
                    query = MysqlParam::add_param(*param, query)
                }
                let t = query.execute(x).await?;
                Ok(ExecuteResult {
                    rows_affected: t.rows_affected(),
                })
            }

            #[cfg(feature = "mssql")]
            TransT::Mssql(inner) => inner.execute(sql, params).await,
        }
    }

    async fn fetch_rows(&self, sql: &str, params: &[&(dyn Param + Sync)]) -> Result<Vec<Row>> {
        let mut inner = self.as_inner_mut();
        match &mut inner {
            #[cfg(feature = "sqlite")]
            TransT::Sqlite(t) => {
                let x: &mut <sqlx::Sqlite as sqlx::Database>::Connection = t;
                let mut query = sqlx::query::<sqlx::Sqlite>(sql);
                for param in params {
                    query = SqliteParam::add_param(*param, query)
                }
                let mut raw_rows = query.fetch_all(x).await?;
                let rows: Vec<Row> = raw_rows.drain(..).map(Row::from).collect();
                Ok(rows)
            }

            #[cfg(feature = "postgres")]
            TransT::Postgres(t) => {
                let x: &mut <sqlx::Postgres as sqlx::Database>::Connection = t;
                let mut query = sqlx::query::<sqlx::Postgres>(sql);
                for param in params {
                    query = PostgresParam::add_param(*param, query)
                }
                let mut raw_rows = query.fetch_all(x).await?;
                let rows: Vec<Row> = raw_rows.drain(..).map(Row::from).collect();
                Ok(rows)
            }

            #[cfg(feature = "mysql")]
            TransT::Mysql(t) => {
                let x: &mut <sqlx::MySql as sqlx::Database>::Connection = t;
                let mut query = sqlx::query::<sqlx::MySql>(sql);
                for param in params {
                    query = MysqlParam::add_param(*param, query)
                }
                let mut raw_rows = query.fetch_all(x).await?;
                let rows: Vec<Row> = raw_rows.drain(..).map(Row::from).collect();
                Ok(rows)
            }

            #[cfg(feature = "mssql")]
            TransT::Mssql(inner) => Ok(inner.fetch_rows(sql, params).await?),
        }
    }
}
