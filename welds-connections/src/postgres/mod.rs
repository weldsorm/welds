use super::transaction::{TransT, Transaction};
use super::{trace, Row};
use super::TransactStart;
use super::{Client, Param};
use crate::errors::Result;
use crate::ExecuteResult;
use async_trait::async_trait;
use sqlx::postgres::PgArguments;
use sqlx::query::Query;
use sqlx::{PgPool, Postgres};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct PostgresClient {
    pool: Arc<PgPool>,
}

#[async_trait]
impl TransactStart for PostgresClient {
    async fn begin<'t>(&'t self) -> Result<Transaction<'t>> {
        let t = self.pool.begin().await?;
        let t = TransT::Postgres(t);
        Ok(Transaction::new(t))
    }
}

pub async fn connect(url: &str) -> Result<PostgresClient> {
    let pool = PgPool::connect(url).await?;
    Ok(PostgresClient {
        pool: Arc::new(pool),
    })
}

impl From<sqlx::PgPool> for PostgresClient {
    fn from(pool: sqlx::PgPool) -> PostgresClient {
        PostgresClient {
            pool: Arc::new(pool),
        }
    }
}

impl PostgresClient {
    pub fn as_sqlx_pool(&self) -> &PgPool {
        &self.pool
    }
}

use sqlx::encode::Encode;
use sqlx::types::Type;

#[async_trait]
impl Client for PostgresClient {
    async fn execute(&self, sql: &str, params: &[&(dyn Param + Sync)]) -> Result<ExecuteResult> {
        log::trace!("POSTGRES EXECUTE: {}", sql);
        let mut query = sqlx::query::<Postgres>(sql);
        for param in params {
            query = PostgresParam::add_param(*param, query);
        }
        let r = trace::db_error(query.execute(&*self.pool).await)?;
        Ok(ExecuteResult {
            rows_affected: r.rows_affected(),
        })
    }

    async fn fetch_rows(&self, sql: &str, params: &[&(dyn Param + Sync)]) -> Result<Vec<Row>> {
        log::trace!("POSTGRES FETCH_ROWS: {}", sql);
        let mut query = sqlx::query::<Postgres>(sql);
        for param in params {
            query = PostgresParam::add_param(*param, query);
        }
        let mut raw_rows = trace::db_error(query.fetch_all(&*self.pool).await)?;
        let rows: Vec<Row> = raw_rows.drain(..).map(Row::from).collect();
        Ok(rows)
    }

    async fn fetch_many<'s, 'args, 't>(
        &self,
        fetches: &[crate::Fetch<'s, 'args, 't>],
    ) -> Result<Vec<Vec<Row>>> {
        let mut datasets = Vec::default();
        let mut conn = self.pool.acquire().await?;
        for fetch in fetches {
            let sql = fetch.sql;
            log::trace!("POSTGRES FETCH_MANY: {}", sql);
            let params = fetch.params;
            let mut query = sqlx::query::<Postgres>(sql);
            for param in params {
                query = PostgresParam::add_param(*param, query);
            }
            let mut raw_rows = trace::db_error(query.fetch_all(&mut *conn).await)?;
            let rows: Vec<Row> = raw_rows.drain(..).map(Row::from).collect();
            datasets.push(rows);
        }
        Ok(datasets)
    }

    fn syntax(&self) -> crate::Syntax {
        crate::Syntax::Postgres
    }
}

pub trait PostgresParam {
    fn add_param<'q>(
        &'q self,
        query: Query<'q, Postgres, PgArguments>,
    ) -> Query<'q, Postgres, PgArguments>;
}

impl<T> PostgresParam for T
where
    for<'a> T: 'a + Send + Encode<'a, Postgres> + Type<Postgres>,
    for<'a> &'a T: Send,
{
    fn add_param<'q>(
        &'q self,
        query: Query<'q, Postgres, PgArguments>,
    ) -> Query<'q, Postgres, PgArguments> {
        query.bind(self)
    }
}
