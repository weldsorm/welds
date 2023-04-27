use anyhow::Result;
use std::sync::Mutex;

pub struct Transaction<'trans, DB: sqlx::Database> {
    pub(crate) inner: Mutex<sqlx::Transaction<'trans, DB>>,
}

impl<'trans, DB: sqlx::Database> Transaction<'trans, DB> {
    pub(crate) async fn new(pool: super::Pool<DB>) -> Result<Transaction<'trans, DB>> {
        let trans = pool.as_sqlx_pool().begin().await?;
        let inner = Mutex::new(trans);
        Ok(Transaction { inner })
    }

    /// Rollback the transaction
    pub async fn rollback(self) -> Result<()> {
        let inner = self.inner.into_inner().unwrap();
        inner.rollback().await?;
        Ok(())
    }
    /// Rollback the transaction
    pub async fn commit(self) -> Result<()> {
        let inner = self.inner.into_inner().unwrap();
        inner.commit().await?;
        Ok(())
    }
}
