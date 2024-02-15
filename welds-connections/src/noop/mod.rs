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
pub struct NoopClient {
    syntax: Syntax,
    last_sql: Arc<Mutex<Option<String>>>,
}

pub fn build(syntax: Syntax) -> NoopClient {
    NoopClient {
        syntax,
        last_sql: Arc::new(Mutex::new(None)),
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
}

#[async_trait]
impl Client for NoopClient {
    async fn execute(&self, sql: &str, _params: &[&(dyn Param + Sync)]) -> Result<ExecuteResult> {
        // save off the sql
        let lock = self.last_sql.clone();
        let mut mutex = lock.lock().unwrap();
        *mutex = Some(sql.to_string());
        // return nothing
        Ok(ExecuteResult { rows_affected: 0 })
    }

    async fn fetch_rows(&self, sql: &str, _params: &[&(dyn Param + Sync)]) -> Result<Vec<Row>> {
        // save off the sql
        let lock = self.last_sql.clone();
        let mut mutex = lock.lock().unwrap();
        *mutex = Some(sql.to_string());
        // return nothing
        Ok(Vec::default())
    }
    fn syntax(&self) -> crate::Syntax {
        self.syntax
    }
}
