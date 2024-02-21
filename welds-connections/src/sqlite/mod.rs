use super::transaction::{TransT, Transaction};
use super::Row;
use super::TransactStart;
use super::{Client, Param};
use crate::errors::Result;
use crate::ExecuteResult;
use async_trait::async_trait;
use sqlx::query::Query;
use sqlx::sqlite::SqliteArguments;
use sqlx::{Sqlite, SqlitePool};
use std::sync::Arc;

pub struct SqliteClient {
    pool: Arc<SqlitePool>,
}

#[async_trait]
impl TransactStart for SqliteClient {
    async fn begin(&self) -> Result<Transaction> {
        let t = self.pool.begin().await?;
        let t = TransT::Sqlite(t);
        Ok(Transaction::new(t))
    }
}

pub async fn get_conn(url: &str) -> Result<SqliteClient> {
    let pool = SqlitePool::connect(url).await?;
    Ok(SqliteClient {
        pool: Arc::new(pool),
    })
}

impl From<sqlx::SqlitePool> for SqliteClient {
    fn from(pool: sqlx::SqlitePool) -> SqliteClient {
        SqliteClient {
            pool: Arc::new(pool),
        }
    }
}

impl SqliteClient {
    pub fn as_sqlx_pool(&self) -> &SqlitePool {
        &self.pool
    }
}

use sqlx::encode::Encode;
use sqlx::types::Type;

#[async_trait]
impl Client for SqliteClient {
    async fn execute(&self, sql: &str, params: &[&(dyn Param + Sync)]) -> Result<ExecuteResult> {
        let mut query = sqlx::query::<Sqlite>(sql);
        for param in params {
            query = SqliteParam::add_param(*param, query);
        }
        let r = query.execute(&*self.pool).await?;
        Ok(ExecuteResult {
            rows_affected: r.rows_affected(),
        })
    }

    async fn fetch_rows(&self, sql: &str, params: &[&(dyn Param + Sync)]) -> Result<Vec<Row>> {
        let mut query = sqlx::query::<Sqlite>(sql);
        for param in params {
            query = SqliteParam::add_param(*param, query);
        }
        let mut raw_rows = query.fetch_all(&*self.pool).await?;
        let rows: Vec<Row> = raw_rows.drain(..).map(Row::from).collect();
        Ok(rows)
    }
    fn syntax(&self) -> crate::Syntax {
        crate::Syntax::Sqlite
    }
}

pub trait SqliteParam {
    fn add_param<'q>(
        &'q self,
        query: Query<'q, Sqlite, SqliteArguments<'q>>,
    ) -> Query<'q, Sqlite, SqliteArguments<'q>>;
}

impl<T> SqliteParam for T
where
    for<'a> T: 'a + Send + Encode<'a, Sqlite> + Type<Sqlite>,
    for<'a> &'a T: Send,
{
    fn add_param<'q>(
        &'q self,
        query: Query<'q, Sqlite, SqliteArguments<'q>>,
    ) -> Query<'q, Sqlite, SqliteArguments<'q>> {
        query.bind(self)
    }
}
