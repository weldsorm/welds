use super::Row;
use super::{Client, Param};
use crate::errors::Result;
use crate::{ExecuteResult, Syntax};
use async_trait::async_trait;
use std::sync::Mutex;

#[cfg(feature = "mssql")]
use crate::mssql::transaction::MssqlTransaction;

pub struct Transaction<'t> {
    inner: Mutex<Option<TransT<'t>>>,
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
            inner: Mutex::new(Some(inner)),
        }
    }

    pub async fn rollback(self) -> Result<()> {
        let inner = self.take_conn();
        inner.rollback().await?;
        Ok(())
    }
    pub async fn commit(self) -> Result<()> {
        let inner = self.take_conn();
        inner.commit().await?;
        Ok(())
    }
}

impl<'t> Transaction<'t> {
    // HACK - CODE SMELL:
    // we need a &mut conn for the connection pool
    // this (take_conn/return_conn) acts like a CellRef
    // It will panic if you try to the conn more one at at time
    //
    fn take_conn(&self) -> TransT<'t> {
        let mut placeholder = None;
        let mut m = self.inner.lock().unwrap();
        let inner: &mut Option<TransT<'t>> = &mut m;
        // Panic if the conn is already taken
        assert!(inner.is_some(), "Pool was already taken");
        std::mem::swap(&mut placeholder, inner);
        placeholder.unwrap()
    }
    fn return_conn(&self, conn: TransT<'t>) {
        let mut placeholder = Some(conn);
        let mut m = self.inner.lock().unwrap();
        let inner: &mut Option<TransT<'t>> = &mut m;
        // Panic if we already have a the conn
        assert!(inner.is_none(), "Overriding existing pool");
        std::mem::swap(&mut placeholder, inner);
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

impl TransT<'_> {
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

#[cfg(feature = "mysql")]
use super::mysql::MysqlParam;
#[cfg(feature = "postgres")]
use super::postgres::PostgresParam;
#[cfg(feature = "sqlite")]
use super::sqlite::SqliteParam;

#[async_trait]
impl Client for Transaction<'_> {
    fn syntax(&self) -> crate::Syntax {
        self.syntax
    }

    async fn execute(&self, sql: &str, params: &[&(dyn Param + Sync)]) -> Result<ExecuteResult> {
        let mut inner = self.take_conn();
        let results = execute_inner(&mut inner, sql, params).await;
        self.return_conn(inner);
        results
    }

    async fn fetch_rows(&self, sql: &str, params: &[&(dyn Param + Sync)]) -> Result<Vec<Row>> {
        let mut inner = self.take_conn();
        let results = fetch_rows_inner(&mut inner, sql, params).await;
        self.return_conn(inner);
        results
    }

    async fn fetch_many<'s, 'args, 'i>(
        &self,
        fetches: &[crate::Fetch<'s, 'args, 'i>],
    ) -> Result<Vec<Vec<Row>>> {
        // transactions are already locked to a single connection.
        // Just run the batch of fetches
        let mut datasets = Vec::default();
        let mut inner = self.take_conn();
        for fetch in fetches {
            let sql = fetch.sql;
            let params = fetch.params;
            let r = fetch_rows_inner(&mut inner, sql, params).await;
            let is_err = r.is_err();
            datasets.push(r);
            if is_err {
                break;
            }
        }
        self.return_conn(inner);
        datasets.drain(..).collect()
    }
}

async fn execute_inner(
    inner: &mut TransT<'_>,
    sql: &str,
    params: &[&(dyn Param + Sync)],
) -> Result<ExecuteResult> {
    match inner {
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
        TransT::Mssql(inner) => {
            let result = inner.execute(sql, params).await;
            if result.is_err() {
                let _ = inner.internal_rollback_check().await;
            }
            result
        }
    }
}

async fn fetch_rows_inner(
    inner: &mut TransT<'_>,
    sql: &str,
    params: &[&(dyn Param + Sync)],
) -> Result<Vec<Row>> {
    match inner {
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
        TransT::Mssql(inner) => {
            let result = inner.fetch_rows(sql, params).await;
            if result.is_err() {
                let _ = inner.internal_rollback_check().await;
            }
            result
        }
    }
}
