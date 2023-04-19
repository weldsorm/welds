use crate::connection::Connection;
use crate::query::clause::DbParam;
use crate::query::{delete, insert, update};
use crate::table::{HasSchema, TableColumns, TableInfo, WriteToArgs};
use crate::writers::column::DbColumnWriter;
use crate::writers::insert::DbInsertWriter;
use anyhow::Result;
use sqlx::database::HasArguments;
use sqlx::IntoArguments;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum DbStatus {
    NotInDatabase,
    NotModified,
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
    sql_buff: String,
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
    /// Returns a DbState<T> that assumes its inner T does not exist in the database
    pub fn new_uncreated(inner: T) -> DbState<T> {
        DbState {
            _t: PhantomData::default(),
            inner,
            status: DbStatus::NotInDatabase,
            sql_buff: String::default(),
        }
    }

    /// Returns a DbState<T> that assumes its inner T already exist in the database
    pub fn db_loaded(inner: T) -> DbState<T> {
        DbState {
            _t: PhantomData::default(),
            inner,
            status: DbStatus::NotModified,
            sql_buff: String::default(),
        }
    }

    /// Saves the inner T to the database. Results in an insert or update if needed. If no change
    /// has been detected on the inner T, No operation will occur
    ///
    pub async fn save<'r, 'args, C, DB>(&'r mut self, conn: &'r C) -> Result<()>
    where
        T: WriteToArgs<DB> + HasSchema + for<'fr> sqlx::FromRow<'fr, DB::Row>,
        DB: sqlx::Database + DbParam + DbInsertWriter + DbColumnWriter,
        <DB as HasArguments<'r>>::Arguments: IntoArguments<'args, DB>,
        <T as HasSchema>::Schema: TableInfo + TableColumns<DB>,
        C: Connection<DB>,
    {
        match self.status {
            DbStatus::NotModified => {}
            DbStatus::Edited => {
                self.sql_buff = String::default();
                update::update_one(&mut self.sql_buff, &self.inner, conn).await?;
            }
            DbStatus::NotInDatabase => {
                self.sql_buff = String::default();
                insert::insert_one(&mut self.sql_buff, &mut self.inner, conn).await?;
            }
        }
        self.status = DbStatus::NotModified;
        Ok(())
    }

    /// Removes the inner T from the database. If T is not in the database no operation will occur
    pub async fn delete<'r, 'args, C, DB>(&'r mut self, conn: &'r C) -> Result<()>
    where
        'r: 'args,
        T: WriteToArgs<DB> + HasSchema,
        DB: sqlx::Database + DbParam + DbInsertWriter + DbColumnWriter,
        C: Connection<DB>,
        <DB as HasArguments<'r>>::Arguments: IntoArguments<'args, DB>,
        <T as HasSchema>::Schema: TableInfo + TableColumns<DB>,
    {
        match self.status {
            DbStatus::NotModified => {
                self.sql_buff = String::default();
                delete::delete_one(&mut self.sql_buff, &self.inner, conn).await?;
            }
            DbStatus::Edited => {
                self.sql_buff = String::default();
                delete::delete_one(&mut self.sql_buff, &self.inner, conn).await?;
            }
            DbStatus::NotInDatabase => {}
        }
        self.status = DbStatus::NotInDatabase;
        Ok(())
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
