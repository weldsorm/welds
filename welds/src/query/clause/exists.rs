use super::ClauseAdder;
use crate::{alias::TableAlias, query::select::SelectBuilder};
use std::cell::RefCell;

/// Used to generated a SQL EXISTS clause for writing sub-queries

pub struct ExistIn<'args, DB> {
    outer_column: String,
    outer_tablealias: RefCell<Option<String>>,
    inner_column: String,
    inner_tablename: String,
    wheres: Vec<Box<dyn ClauseAdder<'args, DB>>>,
    inner_exists_ins: Vec<Self>,
}

impl<'args, DB> ExistIn<'args, DB>
where
    DB: sqlx::Database,
{
    pub(crate) fn new<T>(
        sb: SelectBuilder<'args, T, DB>,
        outer_column: String,
        inner_tablename: String,
        inner_column: String,
    ) -> Self {
        ExistIn::<'args, DB> {
            outer_column,
            outer_tablealias: RefCell::new(None),
            inner_column,
            inner_tablename,
            wheres: sb.wheres, //wheres: sb.wheres,
            inner_exists_ins: sb.exist_ins,
        }
    }

    pub(crate) fn set_outer_tablealias(&self, tablealias: &str) {
        self.outer_tablealias.replace(Some(tablealias.to_owned()));
    }

    fn inner_fk_equal(&self, inner_tablealias: &str) -> String {
        let cell = self.outer_tablealias.borrow();
        let outer_tablealias = cell.as_ref().unwrap();
        format!(
            "{}.{} = {}.{}",
            inner_tablealias, self.inner_column, outer_tablealias, self.outer_column
        )
    }

    fn exists_clause(&self, inner_tablealias: &str, inner_clauses: &str) -> String {
        format!(
            "EXISTS ( SELECT {} FROM {} {} WHERE {} )",
            self.inner_column, self.inner_tablename, inner_tablealias, inner_clauses
        )
    }
}

impl<'args, DB> ClauseAdder<'args, DB> for ExistIn<'args, DB>
where
    DB: sqlx::Database,
{
    fn bind(&self, args: &mut <DB as sqlx::database::HasArguments<'args>>::Arguments) {
        for w in &self.wheres {
            w.bind(args);
        }
        for w in &self.inner_exists_ins {
            w.bind(args);
        }
    }

    fn clause(&self, alias: &TableAlias, next_params: &super::NextParam) -> Option<String> {
        let self_tablealias = alias.peek();
        let mut inner_wheres: Vec<String> = self
            .wheres
            .iter()
            .map(|w| w.clause(alias, next_params))
            .filter_map(|x| x)
            .collect();
        inner_wheres.push(self.inner_fk_equal(&self_tablealias));

        // exists inside this exist clause
        for ins in &self.inner_exists_ins {
            alias.bump();
            ins.set_outer_tablealias(&self_tablealias);
            if let Some(more) = ins.clause(alias, next_params) {
                inner_wheres.push(more);
            }
        }

        let inner_clauses = inner_wheres.join(" AND ");
        Some(self.exists_clause(&self_tablealias, &inner_clauses))
    }
}
