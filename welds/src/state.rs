//use crate::query::{delete, insert, update};
use crate::errors::Result;
use crate::model_traits::{
    ColumnDefaultCheck, HasSchema, TableColumns, TableInfo, UpdateFromRow, WriteToArgs,
};
use crate::query::delete;
use crate::query::insert;
use crate::query::update;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use welds_connections::Client;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DbStatus {
    /// The entity has NOT been saved.
    NotInDatabase,
    /// The entity is an exact copy of what is in the database
    NotModified,
    /// The entity is most likely different from what is in the database
    Edited,
}

/// Wraps a T to keep track of changes and current state in the database
///
/// Also used to Save changes. Save will result in a create or update as needed.
/// delete will remove from the database.
pub struct DbState<T> {
    _t: PhantomData<T>,
    inner: T,
    status: DbStatus,
}

impl<T> std::fmt::Debug for DbState<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.inner, f)
    }
}

impl<T> DbState<T> {
    /// Returns status of the entity. If it is in the database/unsaved/modified/..
    pub fn db_status(&self) -> DbStatus {
        self.status
    }

    /// Returns a DbState<T> that assumes its inner T does not exist in the database
    pub fn new_uncreated(inner: T) -> DbState<T> {
        DbState {
            _t: PhantomData,
            inner,
            status: DbStatus::NotInDatabase,
        }
    }

    /// Returns a DbState<T> that assumes its inner T already exist in the database
    pub fn db_loaded(inner: T) -> DbState<T> {
        DbState {
            _t: PhantomData,
            inner,
            status: DbStatus::NotModified,
        }
    }

    /// Saves the inner T to the database. Results in an insert or update if needed. If no change
    /// has been detected on the inner T, No operation will occur
    ///
    pub async fn save(&mut self, client: &dyn Client) -> Result<()>
    where
        T: HasSchema + WriteToArgs + ColumnDefaultCheck,
        <T as HasSchema>::Schema: TableInfo + TableColumns,
        T: UpdateFromRow,
    {
        match self.status {
            DbStatus::NotModified => {}
            DbStatus::Edited => {
                update::update_one(&mut self.inner, client).await?;
            }
            DbStatus::NotInDatabase => {
                insert::insert_one(&mut self.inner, client).await?;
            }
        }
        self.status = DbStatus::NotModified;
        Ok(())
    }

    /// Removes the inner T from the database. If T is not in the database no operation will occur
    pub async fn delete(&mut self, client: &dyn Client) -> Result<()>
    where
        T: HasSchema + WriteToArgs,
        <T as HasSchema>::Schema: TableInfo + TableColumns,
    {
        match self.status {
            DbStatus::NotModified => {
                delete::delete_one(&self.inner, client).await?;
            }
            DbStatus::Edited => {
                delete::delete_one(&self.inner, client).await?;
            }
            DbStatus::NotInDatabase => {}
        }
        self.status = DbStatus::NotInDatabase;
        Ok(())
    }

    /// Consumes the DbState, returning the wrapped value. The inner value is nolonger connected to
    /// welds and can nolonger be saved/created/deleted
    pub fn into_inner(self) -> T {
        self.inner
    }

    /// Consumes the model and transforms it into an immutable object that is useful for Views and sharing.
    pub fn into_vm(self) -> std::sync::Arc<T> {
        std::sync::Arc::new(self.inner)
    }

    /// Overwrite the inner value with another, and set the db state ready for update.
    /// 
    /// ⚠️ It may update the wrong row if the Primary Key is modified. Make sure to check beforehand. ⚠️
    pub fn replace_inner(&mut self, new: T) {
        if self.status == DbStatus::NotModified {
            self.status = DbStatus::Edited
        }
        self.inner = new;
    }
}

impl<T> Deref for DbState<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for DbState<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.status == DbStatus::NotModified {
            self.status = DbStatus::Edited
        }
        &mut self.inner
    }
}
