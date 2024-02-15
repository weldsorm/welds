use anyhow::Result;
use std::sync::Mutex;

pub struct Transaction<'trans, DB: sqlx::Database> {
    inner: Mutex<sqlx::Transaction<'trans, DB>>,
}

impl<'trans, DB: sqlx::Database> Transaction<'trans, DB> {
    pub(crate) async fn new(pool: super::Pool<DB>) -> Result<Transaction<'trans, DB>> {
        let trans = pool.as_sqlx_pool().begin().await?;
        Ok(Transaction {
            inner: Mutex::new(trans),
        })
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

    /// returns a mut ref to the trans to run the queries with.
    /// HACK: Only allow use in impl of connection::Connection
    #[allow(clippy::mut_from_ref)]
    pub(crate) fn as_inner_mut<'a>(&'a self) -> &'a mut sqlx::Transaction<'trans, DB> {
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
        let b: &sqlx::Transaction<DB> = &lock;
        let ptr: *const sqlx::Transaction<DB> = b;
        unsafe {
            let ptr_mut = ptr as *mut sqlx::Transaction<DB>;
            &mut *ptr_mut
        }
    }
}
