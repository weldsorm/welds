use super::ClauseAdder;
use crate::query::clause::OrderBy;
use crate::writers::limit_skip::DbLimitSkipWriter;
use crate::{alias::TableAlias, query::builder::QueryBuilder};
use std::cell::RefCell;

/// Used to generated a SQL EXISTS clause for writing sub-queries

pub struct ExistIn<'args, DB> {
    outer_column: String,
    outer_tablealias: RefCell<Option<String>>,
    inner_column: String,
    inner_tablename: String,
    wheres: Vec<Box<dyn ClauseAdder<'args, DB>>>,
    inner_exists_ins: Vec<Self>,
    limit: Option<i64>,
    offset: Option<i64>,
    orderby: Vec<OrderBy>,
}

impl<'args, DB> ExistIn<'args, DB>
where
    DB: sqlx::Database + DbLimitSkipWriter,
{
    pub(crate) fn new<T>(
        sb: QueryBuilder<'args, T, DB>,
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
            limit: sb.limit,
            offset: sb.offset,
            orderby: sb.orderby,
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

    fn tails(&self) -> String {
        use crate::query::tail;
        tail::write::<DB>(&self.limit, &self.offset, &self.orderby).unwrap_or_default()
    }

    fn exists_clause(&self, inner_tablealias: &str, inner_clauses: &str) -> String {
        let tails = self.tails();
        format!(
            "EXISTS ( SELECT {} FROM {} {} WHERE {} {})",
            self.inner_column, self.inner_tablename, inner_tablealias, inner_clauses, tails
        )
    }
}

impl<'args, DB> ClauseAdder<'args, DB> for ExistIn<'args, DB>
where
    DB: sqlx::Database + DbLimitSkipWriter,
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
            .filter_map(|w| w.clause(alias, next_params))
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
