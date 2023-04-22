use super::Connection;
use super::Pool;
use super::Transaction;
use anyhow::Result;
use async_trait::async_trait;
use sqlx::database::HasArguments;
use sqlx::query::QueryAs;
use sqlx::Sqlite;

type DbType = Sqlite;

/// This file contains all the Sqlite impl for connection and transaction

pub async fn connect_sqlite(connection_string: &str) -> Result<Pool<DbType>> {
    let sqlx_pool = sqlx::SqlitePool::connect(connection_string).await?;
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

#[async_trait(?Send)]
impl Connection<DbType> for Pool<DbType> {
    async fn execute<'a>(
        &'a self,
        sql: &'a str,
        args: <DbType as HasArguments<'a>>::Arguments,
    ) -> Result<()> {
        let mut x = self.inner.acquire().await?;
        let q = sqlx::query_with(sql, args);
        q.execute(&mut x).await?;
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
        let q: QueryAs<DbType, T, <DbType as HasArguments>::Arguments> =
            sqlx::query_as_with(sql, args);
        let rows = q.fetch_all(&mut x).await?;
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
        let q: QueryAs<DbType, T, <DbType as HasArguments>::Arguments> =
            sqlx::query_as_with(sql, args);
        let row = q.fetch_optional(&mut x).await?;
        Ok(row)
    }

    /// Returns the un-parsed rows
    async fn fetch_rows<'a>(
        &'a self,
        sql: &'a str,
        args: <DbType as HasArguments<'a>>::Arguments,
    ) -> Result<Vec<<DbType as sqlx::Database>::Row>> {
        let mut x = self.inner.acquire().await?;
        let query = sqlx::query_with(sql, args);
        let rows = query.fetch_all(&mut x).await?;
        Ok(rows)
    }

    /// Returns the un-parsed rows from all the sql statements given
    async fn fetch_many_rows<'a>(
        &'a self,
        statments: Vec<(&'a str, <DbType as HasArguments<'a>>::Arguments)>,
    ) -> Result<Vec<<DbType as sqlx::Database>::Row>> {
        let mut x = self.inner.acquire().await?;
        let mut rows = Vec::default();
        for (sql, args) in statments {
            let query = sqlx::query_with(sql, args);
            let mut batch = query.fetch_all(&mut x).await?;
            rows.append(&mut batch);
        }
        Ok(rows)
    }

    /// Returns what type of DB you are connected with
    fn provider(&self) -> super::DbProvider {
        super::DbProvider::Sqlite
    }
}

#[async_trait(?Send)]
impl<'trans> Connection<DbType> for Transaction<'trans, DbType> {
    async fn execute<'a>(
        &'a self,
        sql: &'a str,
        args: <DbType as HasArguments<'a>>::Arguments,
    ) -> Result<()> {
        let mut t = self.inner.try_borrow_mut()?;
        let trans: &mut sqlx::Transaction<DbType> = &mut t;
        let q = sqlx::query_with(sql, args);
        q.execute(trans).await?;
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
        let mut t = self.inner.try_borrow_mut()?;
        let trans: &mut sqlx::Transaction<DbType> = &mut t;
        let q: QueryAs<DbType, T, <DbType as HasArguments>::Arguments> =
            sqlx::query_as_with(sql, args);
        let rows = q.fetch_all(trans).await?;
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
        let mut t = self.inner.try_borrow_mut()?;
        let trans: &mut sqlx::Transaction<DbType> = &mut t;
        let q: QueryAs<DbType, T, <DbType as HasArguments>::Arguments> =
            sqlx::query_as_with(sql, args);
        let row = q.fetch_optional(trans).await?;
        Ok(row)
    }

    /// Returns the un-parsed rows
    async fn fetch_rows<'a>(
        &'a self,
        sql: &'a str,
        args: <DbType as HasArguments<'a>>::Arguments,
    ) -> Result<Vec<<DbType as sqlx::Database>::Row>> {
        let mut t = self.inner.try_borrow_mut()?;
        let trans: &mut sqlx::Transaction<DbType> = &mut t;
        let query = sqlx::query_with(sql, args);
        let rows = query.fetch_all(trans).await?;
        Ok(rows)
    }

    /// Returns the un-parsed rows from all the sql statements given
    async fn fetch_many_rows<'a>(
        &'a self,
        statments: Vec<(&'a str, <DbType as HasArguments<'a>>::Arguments)>,
    ) -> Result<Vec<<DbType as sqlx::Database>::Row>> {
        let mut t = self.inner.try_borrow_mut()?;
        let trans: &mut sqlx::Transaction<DbType> = &mut t;
        let mut rows = Vec::default();
        for (sql, args) in statments {
            let query = sqlx::query_with(sql, args);
            let mut batch = query.fetch_all(&mut *trans).await?;
            rows.append(&mut batch);
        }
        Ok(rows)
    }

    /// Returns what type of DB you are connected with
    fn provider(&self) -> super::DbProvider {
        super::DbProvider::Sqlite
    }
}
