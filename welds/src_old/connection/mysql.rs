use super::Connection;
use super::Pool;
use super::Transaction;
use anyhow::Result;
use async_trait::async_trait;
use sqlx::database::HasArguments;
use sqlx::query::QueryAs;
use sqlx::MySql;

type DbType = MySql;
fn provider() -> super::DbProvider {
    super::DbProvider::MySql
}

/// This file contains all the MySql impl for connection and transaction

pub async fn connect_mysql(connection_string: &str) -> Result<Pool<DbType>> {
    let sqlx_pool = sqlx::MySqlPool::connect(connection_string).await?;
    Ok(sqlx_pool.into())
}

impl From<sqlx::Pool<DbType>> for Pool<DbType> {
    fn from(inner: sqlx::Pool<DbType>) -> Self {
        Pool { inner }
    }
}
impl From<&sqlx::Pool<DbType>> for Pool<DbType> {
    fn from(inner: &sqlx::Pool<DbType>) -> Self {
        Pool {
            inner: inner.clone(),
        }
    }
}

#[async_trait]
impl Connection<DbType> for Pool<DbType> {
    async fn execute<'a>(
        &'a self,
        sql: &'a str,
        args: <DbType as HasArguments<'a>>::Arguments,
    ) -> Result<()> {
        let mut x = self.inner.acquire().await?;
        let x = &mut *x;
        let q = sqlx::query_with(sql, args);
        q.execute(x).await?;
        Ok(())
    }

    async fn fetch_all<'a, T>(
        &'a self,
        sql: &'a str,
        args: <DbType as HasArguments<'a>>::Arguments,
    ) -> Result<Vec<T>>
    where
        T: Send + Unpin + for<'r> sqlx::FromRow<'r, <DbType as sqlx::Database>::Row>,
    {
        let mut x = self.inner.acquire().await?;
        let x = &mut *x;
        let q: QueryAs<DbType, T, <DbType as HasArguments>::Arguments> =
            sqlx::query_as_with(sql, args);
        let rows = q.fetch_all(x).await?;
        Ok(rows)
    }

    async fn fetch_one<'a, T>(
        &'a self,
        sql: &'a str,
        args: <DbType as HasArguments<'a>>::Arguments,
    ) -> Result<Option<T>>
    where
        T: Send + Unpin + for<'r> sqlx::FromRow<'r, <DbType as sqlx::Database>::Row>,
    {
        let mut x = self.inner.acquire().await?;
        let x = &mut *x;
        let q: QueryAs<DbType, T, <DbType as HasArguments>::Arguments> =
            sqlx::query_as_with(sql, args);
        let row = q.fetch_optional(x).await?;
        Ok(row)
    }

    /// Returns the un-parsed rows
    async fn fetch_rows<'a>(
        &'a self,
        sql: &'a str,
        args: <DbType as HasArguments<'a>>::Arguments,
    ) -> Result<Vec<<DbType as sqlx::Database>::Row>> {
        let mut x = self.inner.acquire().await?;
        let x = &mut *x;
        let query = sqlx::query_with(sql, args);
        let rows = query.fetch_all(x).await?;
        Ok(rows)
    }

    /// Returns the un-parsed rows from all the sql statements given
    async fn fetch_many_rows<'a>(
        &'a self,
        statments: Vec<(&'a str, <DbType as HasArguments<'a>>::Arguments)>,
    ) -> Result<Vec<<DbType as sqlx::Database>::Row>> {
        let mut pc = self.inner.acquire().await?;
        let mut rows = Vec::default();
        for (sql, args) in statments {
            let x = &mut *pc;
            let query = sqlx::query_with(sql, args);
            let mut batch = query.fetch_all(x).await?;
            rows.append(&mut batch);
        }
        Ok(rows)
    }

    /// Returns what type of DB you are connected with
    fn provider(&self) -> super::DbProvider {
        provider()
    }
}

#[async_trait]
impl<'trans> Connection<DbType> for Transaction<'trans, DbType> {
    async fn execute<'a>(
        &'a self,
        sql: &'a str,
        args: <DbType as HasArguments<'a>>::Arguments,
    ) -> Result<()> {
        let x: &mut <DbType as sqlx::Database>::Connection = self.as_inner_mut();
        let q = sqlx::query_with(sql, args);
        q.execute(x).await?;
        Ok(())
    }

    async fn fetch_all<'a, T>(
        &'a self,
        sql: &'a str,
        args: <DbType as HasArguments<'a>>::Arguments,
    ) -> Result<Vec<T>>
    where
        T: Send + Unpin + for<'r> sqlx::FromRow<'r, <DbType as sqlx::Database>::Row>,
    {
        let x: &mut <DbType as sqlx::Database>::Connection = self.as_inner_mut();
        let q: QueryAs<DbType, T, <DbType as HasArguments>::Arguments> =
            sqlx::query_as_with(sql, args);
        let rows = q.fetch_all(x).await?;
        Ok(rows)
    }

    async fn fetch_one<'a, T>(
        &'a self,
        sql: &'a str,
        args: <DbType as HasArguments<'a>>::Arguments,
    ) -> Result<Option<T>>
    where
        T: Send + Unpin + for<'r> sqlx::FromRow<'r, <DbType as sqlx::Database>::Row>,
    {
        let x: &mut <DbType as sqlx::Database>::Connection = self.as_inner_mut();
        let q: QueryAs<DbType, T, <DbType as HasArguments>::Arguments> =
            sqlx::query_as_with(sql, args);
        let row = q.fetch_optional(x).await?;
        Ok(row)
    }

    /// Returns the un-parsed rows
    async fn fetch_rows<'a>(
        &'a self,
        sql: &'a str,
        args: <DbType as HasArguments<'a>>::Arguments,
    ) -> Result<Vec<<DbType as sqlx::Database>::Row>> {
        let x: &mut <DbType as sqlx::Database>::Connection = self.as_inner_mut();
        let query = sqlx::query_with(sql, args);
        let rows = query.fetch_all(x).await?;
        Ok(rows)
    }

    /// Returns the un-parsed rows from all the sql statements given
    async fn fetch_many_rows<'a>(
        &'a self,
        statments: Vec<(&'a str, <DbType as HasArguments<'a>>::Arguments)>,
    ) -> Result<Vec<<DbType as sqlx::Database>::Row>> {
        let mut rows = Vec::default();
        for (sql, args) in statments {
            let x: &mut <DbType as sqlx::Database>::Connection = self.as_inner_mut();
            let query = sqlx::query_with(sql, args);
            let mut batch = query.fetch_all(x).await?;
            rows.append(&mut batch);
        }
        Ok(rows)
    }

    /// Returns what type of DB you are connected with
    fn provider(&self) -> super::DbProvider {
        provider()
    }
}