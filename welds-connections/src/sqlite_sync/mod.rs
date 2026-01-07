use super::TransactStart;
use super::transaction::{TransT, Transaction};
use super::{Client, Param};
use super::{Row, trace};
use crate::ExecuteResult;
use crate::errors::Result;
use rusqlite::types::{FromSql, ToSql, ValueRef};
use std::sync::MutexGuard;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct SqliteClient {
    conn: Arc<Mutex<rusqlite::Connection>>,
}

// required to own the row
pub struct SqliteSyncOwnedRow {
    pub data: Vec<rusqlite::types::Value>, // rusqlite::types::Value is owned
    pub columns: Arc<Vec<String>>,
}

impl SqliteSyncOwnedRow {
    pub fn try_get<T>(&self, idx: usize) -> rusqlite::Result<T>
    where
        T: FromSql,
    {
        let value = &self.data[idx];
        let value_ref = ValueRef::from(value);
        Ok(T::column_result(value_ref)?)
    }
}

impl Client for SqliteClient {
    fn execute(&self, sql: &str, params: &[&(dyn Param + Sync)]) -> Result<ExecuteResult> {
        log::trace!("SQLITE EXECUTE: {}", sql);
        let mut p = Vec::new();
        for param in params {
            p.push(SqliteSyncParam::to_sql_dyn(param));
        }
        let r = trace::db_error(self.conn.lock().unwrap().execute(sql, &*p))?;
        Ok(ExecuteResult::new(r as u64))
    }

    fn fetch_rows(&self, sql: &str, params: &[&(dyn Param + Sync)]) -> Result<Vec<Row>> {
        log::trace!("SQLITE FETCH_ROWS: {}", sql);
        let mut p = Vec::new();
        for param in params {
            p.push(SqliteSyncParam::to_sql_dyn(param));
        }
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(sql)?;
        let column_names: Vec<String> = stmt.column_names().iter().map(|s| s.to_string()).collect();
        let columns = Arc::new(column_names);

        let mut raw_rows = trace::db_error(stmt.query(&*p))?;
        let mut res = Vec::new();

        while let Some(row) = raw_rows.next()? {
            let mut data = Vec::new();
            for i in 0..row.as_ref().column_count() {
                data.push(row.get::<_, rusqlite::types::Value>(i)?);
            }
            res.push(Row::from(SqliteSyncOwnedRow {
                data,
                columns: Arc::clone(&columns),
            }));
        }
        Ok(res)
    }

    fn fetch_many<'s, 'args, 't>(
        &self,
        fetches: &[crate::Fetch<'s, 'args, 't>],
    ) -> Result<Vec<Vec<Row>>> {
        let mut datasets = Vec::default();
        for fetch in fetches {
            let sql = fetch.sql;
            log::trace!("SQLITE FETCH_MANY: {}", sql);
            let params = fetch.params;
            let rows = self.fetch_rows(sql, params)?;
            datasets.push(rows);
        }
        Ok(datasets)
    }

    fn syntax(&self) -> crate::Syntax {
        crate::Syntax::Sqlite
    }
}

pub struct SqliteSyncTransaction<'a> {
    _guard: MutexGuard<'a, rusqlite::Connection>,
    pub transaction: rusqlite::Transaction<'a>,
}

impl TransactStart for SqliteClient {
    fn begin<'t>(&'t self) -> Result<Transaction<'t>> {
        let mut guard = self.conn.lock().unwrap();
        // We need to create the transaction from the guard
        // This requires unsafe because we're creating a self-referential struct
        let transaction = unsafe {
            let conn_ptr = &mut *guard as *mut rusqlite::Connection;
            (*conn_ptr).transaction()?
        };
        let t = SqliteSyncTransaction {
            _guard: guard,
            transaction,
        };
        let t = TransT::SqliteSync(t);
        Ok(Transaction::new(t))
    }
}

pub fn connect(url: &str) -> Result<SqliteClient> {
    let path = url.trim_start_matches("sqlite://");
    let client = rusqlite::Connection::open(path)?;
    Ok(SqliteClient {
        conn: Arc::new(Mutex::new(client)),
    })
}

pub trait SqliteSyncParam {
    fn to_sql_dyn(&self) -> &dyn ToSql;
}

impl<T> SqliteSyncParam for T
where
    T: ToSql,
{
    fn to_sql_dyn(&self) -> &dyn ToSql {
        self
    }
}
