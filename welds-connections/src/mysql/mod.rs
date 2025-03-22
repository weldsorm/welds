use super::transaction::{TransT, Transaction};
use super::TransactStart;
use super::{trace, Row};
use super::{Client, Param};
use crate::errors::Result;
use crate::ExecuteResult;
use async_trait::async_trait;
use sqlx::mysql::MySqlArguments;
use sqlx::query::Query;
use sqlx::{MySql, MySqlPool};
use std::sync::Arc;

#[derive(Clone)]
pub struct MysqlClient {
    pool: Arc<MySqlPool>,
}

#[async_trait]
impl TransactStart for MysqlClient {
    async fn begin<'t>(&'t self) -> Result<Transaction<'t>> {
        let t = self.pool.begin().await?;
        let t = TransT::Mysql(t);
        Ok(Transaction::new(t))
    }
}

pub async fn connect(url: &str) -> Result<MysqlClient> {
    let pool = MySqlPool::connect(url).await?;
    Ok(MysqlClient {
        pool: Arc::new(pool),
    })
}

impl From<sqlx::MySqlPool> for MysqlClient {
    fn from(pool: sqlx::MySqlPool) -> MysqlClient {
        MysqlClient {
            pool: Arc::new(pool),
        }
    }
}

impl MysqlClient {
    pub fn as_sqlx_pool(&self) -> &MySqlPool {
        &self.pool
    }
}

use sqlx::encode::Encode;
use sqlx::types::Type;

#[async_trait]
impl Client for MysqlClient {
    async fn execute(&self, sql: &str, params: &[&(dyn Param + Sync)]) -> Result<ExecuteResult> {
        log::trace!("MYSQL EXECUTE: {}", sql);
        let mut query = sqlx::query::<MySql>(sql);
        for param in params {
            query = MysqlParam::add_param(*param, query);
        }
        let r = trace::db_error(query.execute(&*self.pool).await)?;
        Ok(ExecuteResult {
            rows_affected: r.rows_affected(),
        })
    }

    async fn fetch_rows(&self, sql: &str, params: &[&(dyn Param + Sync)]) -> Result<Vec<Row>> {
        log::trace!("MYSQL FETCH_ROWS: {}", sql);
        let mut query = sqlx::query::<MySql>(sql);
        for param in params {
            query = MysqlParam::add_param(*param, query);
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
            log::trace!("MYSQL FETCH_MANY: {}", sql);
            let params = fetch.params;
            let mut query = sqlx::query::<MySql>(sql);
            for param in params {
                query = MysqlParam::add_param(*param, query);
            }
            let mut raw_rows = trace::db_error(query.fetch_all(&mut *conn).await)?;
            let rows: Vec<Row> = raw_rows.drain(..).map(Row::from).collect();
            datasets.push(rows);
        }
        Ok(datasets)
    }

    fn syntax(&self) -> crate::Syntax {
        crate::Syntax::Mysql
    }
}

pub trait MysqlParam {
    fn add_param<'q>(
        &'q self,
        query: Query<'q, MySql, MySqlArguments>,
    ) -> Query<'q, MySql, MySqlArguments>;
}

impl<T> MysqlParam for T
where
    for<'a> T: 'a + Send + Encode<'a, MySql> + Type<MySql>,
    for<'a> &'a T: Send,
{
    fn add_param<'q>(
        &'q self,
        query: Query<'q, MySql, MySqlArguments>,
    ) -> Query<'q, MySql, MySqlArguments> {
        query.bind(self)
    }
}
