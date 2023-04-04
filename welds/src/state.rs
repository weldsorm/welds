use crate::errors::Result;
use crate::query::clause::DbParam;
use crate::query::{insert, update};
use crate::table::{HasSchema, TableColumns, TableInfo, WriteToArgs};
use crate::writers::column::DbColumnWriter;
use crate::writers::insert::DbInsertWriter;
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
    pub fn new_uncreated(inner: T) -> DbState<T> {
        DbState {
            _t: PhantomData::default(),
            inner,
            status: DbStatus::NotInDatabase,
            sql_buff: String::default(),
        }
    }

    pub fn db_loaded(inner: T) -> DbState<T> {
        DbState {
            _t: PhantomData::default(),
            inner,
            status: DbStatus::NotModified,
            sql_buff: String::default(),
        }
    }

    pub async fn save<'q, 'schema, 'args, 'e, E, DB>(&'q mut self, exec: &'e mut E) -> Result<()>
    where
        E: 'e,
        'q: 'args,
        'schema: 'args,
        T: WriteToArgs<DB> + HasSchema + for<'r> sqlx::FromRow<'r, DB::Row>,
        &'e mut E: sqlx::Executor<'e, Database = DB>,
        DB: sqlx::Database + DbParam + DbInsertWriter + DbColumnWriter,
        <DB as HasArguments<'schema>>::Arguments: IntoArguments<'args, DB>,
        <T as HasSchema>::Schema: TableInfo + TableColumns<DB>,
    {
        match self.status {
            DbStatus::NotModified => {}
            DbStatus::Edited => {
                self.sql_buff = String::default();
                update::update_one(&mut self.sql_buff, &self.inner, exec).await?;
            }
            DbStatus::NotInDatabase => {
                self.sql_buff = String::default();
                insert::insert_one(&mut self.sql_buff, &mut self.inner, exec).await?;
            }
        }
        self.status = DbStatus::NotModified;
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
