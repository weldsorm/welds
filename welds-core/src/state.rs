use crate::errors::Result;
use crate::query::clause::DbParam;
use crate::query::update;
use crate::table::{HasSchema, TableColumns, TableInfo, WriteToArgs};
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

impl<T> DbState<T> {
    pub(crate) fn db_loaded(inner: T) -> DbState<T> {
        DbState {
            _t: PhantomData::default(),
            inner,
            status: DbStatus::NotModified,
            sql_buff: String::default(),
        }
    }

    pub async fn save<'q, 'schema, 'args, 'e, E, DB>(&'q mut self, exec: E) -> Result<()>
    where
        'q: 'args,
        'schema: 'args,
        T: WriteToArgs<DB>,
        E: sqlx::Executor<'e, Database = DB>,
        DB: sqlx::Database + DbParam,
        T: HasSchema, //T: HasSchema + WriteToArgs<DB>,
        <DB as HasArguments<'schema>>::Arguments: IntoArguments<'args, DB>,
        <T as HasSchema>::Schema: TableInfo + TableColumns<DB>,
    {
        match self.status {
            DbStatus::NotModified => {}
            DbStatus::Edited => {
                update::update_one(&mut self.sql_buff, &self.inner, exec).await?;
            }
            DbStatus::NotInDatabase => {
                todo!("impl creating");
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
