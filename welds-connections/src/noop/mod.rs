use super::Row;
use super::{Client, Param};
use crate::errors::Result;
use crate::{ExecuteResult, Syntax};
use async_trait::async_trait;
use std::sync::{Arc, Mutex};

// This is a version of a client that does nothing.
// It is used for testing
//
// You can change its syntax and get the last SQL that we "ran"
#[derive(Clone)]
pub struct NoopClient {
    syntax: Syntax,
    last_sql: Arc<Mutex<Option<String>>>,
    args_count: Arc<Mutex<Option<u64>>>,
}

pub fn build(syntax: Syntax) -> NoopClient {
    NoopClient {
        syntax,
        last_sql: Arc::new(Mutex::new(None)),
        args_count: Arc::new(Mutex::new(None)),
    }
}

impl NoopClient {
    pub fn set_syntax(&mut self, syntax: Syntax) {
        self.syntax = syntax;
    }

    pub fn last_sql(&self) -> Option<String> {
        let lock = self.last_sql.clone();
        let mutex = lock.lock().unwrap();
        mutex.clone()
    }

    pub fn args_count(&self) -> Option<u64> {
        let lock = self.args_count.clone();
        let mutex = lock.lock().unwrap();
        *mutex
    }
}

#[async_trait]
impl Client for NoopClient {
    async fn execute(&self, sql: &str, params: &[&(dyn Param + Sync)]) -> Result<ExecuteResult> {
        // save off the sql
        let lock = self.last_sql.clone();
        let mut mutex = lock.lock().unwrap();
        *mutex = Some(sql.to_string());

        // save off the args count
        let lock = self.args_count.clone();
        let mut mutex = lock.lock().unwrap();
        *mutex = Some(params.len() as u64);

        // return nothing
        Ok(ExecuteResult { rows_affected: 0 })
    }

    async fn fetch_rows(&self, sql: &str, params: &[&(dyn Param + Sync)]) -> Result<Vec<Row>> {
        // save off the sql
        let lock = self.last_sql.clone();
        let mut mutex = lock.lock().unwrap();
        *mutex = Some(sql.to_string());

        // save off the args count
        let lock = self.args_count.clone();
        let mut mutex = lock.lock().unwrap();
        *mutex = Some(params.len() as u64);

        // return nothing
        Ok(Vec::default())
    }

    async fn fetch_many<'s, 'args, 'i>(
        &self,
        fetches: &[crate::Fetch<'s, 'args, 'i>],
    ) -> Result<Vec<Vec<Row>>> {
        let mut total = 0;
        let mut sqls = Vec::default();
        for fetch in fetches {
            sqls.push(fetch.sql);
            total += fetch.params.len()
        }

        // save off the sql
        let lock = self.last_sql.clone();
        let mut mutex = lock.lock().unwrap();
        *mutex = Some(sqls.join(";").to_string());

        // save off the args count
        let lock = self.args_count.clone();
        let mut mutex = lock.lock().unwrap();
        *mutex = Some(total as u64);

        Ok(Vec::default())
    }

    fn syntax(&self) -> crate::Syntax {
        self.syntax
    }
}
