use super::ClauseAdder;
use crate::query::clause::OrderBy;
use crate::writers::limit_skip::DbLimitSkipWriter;
use crate::{alias::TableAlias, query::builder::QueryBuilder};
use std::rc::Rc;

/// Used to generated a SQL EXISTS OR IN clause for writing sub-queries

pub struct ExistIn<'args, DB> {
    outer_column: String,
    inner_column: String,
    inner_tablename: String,
    pub(crate) inner_tablealias: String,
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
            inner_column,
            inner_tablename,
            inner_tablealias: sb.alias,
            wheres: sb.wheres,
            inner_exists_ins: sb.exist_ins,
            limit: sb.limit,
            offset: sb.offset,
            orderby: sb.orderby,
        }
    }

    // re-assign all the alias and alias for sub-tables
    pub(crate) fn set_aliases(&mut self, alias_asigner: &Rc<TableAlias>) {
        self.inner_tablealias = alias_asigner.next();
        for sub in &mut self.inner_exists_ins {
            sub.set_aliases(alias_asigner);
        }
    }

    fn inner_fk_equal(&self, tablealias: &str) -> String {
        format!(
            "{}.{} = {}.{}",
            self.inner_tablealias, self.inner_column, tablealias, self.outer_column
        )
    }

    fn tails(&self) -> String {
        use crate::query::tail;
        tail::write::<DB>(&self.limit, &self.offset, &self.orderby).unwrap_or_default()
    }

    fn exists_clause(&self, _tablealias: &str, inner_clauses: &str) -> String {
        let tails = self.tails();
        format!(
            "EXISTS ( SELECT {} FROM {} {} WHERE {} {})",
            self.inner_column, self.inner_tablename, self.inner_tablealias, inner_clauses, tails
        )
    }

    fn in_clause(&self, tablealias: &str, inner_clauses: &str) -> String {
        let outcol = format!("{}.{}", tablealias, self.outer_column);
        let innercol = format!("{}.{}", self.inner_tablealias, self.inner_column);
        let tails = self.tails();
        let mut wheres = "".to_string();
        if !inner_clauses.is_empty() {
            wheres = format!("WHERE {}", inner_clauses);
        }
        format!(
            " {} IN (SELECT {} FROM {} {} {} {}) ",
            outcol, innercol, self.inner_tablename, self.inner_tablealias, wheres, tails
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

    fn clause(&self, alias: &str, next_params: &super::NextParam) -> Option<String> {
        let using_in = self.limit.is_some();
        let self_tablealias = alias;
        let mut inner_wheres: Vec<String> = self
            .wheres
            .iter()
            .filter_map(|w| w.clause(&self.inner_tablealias, next_params))
            .collect();

        if !using_in {
            inner_wheres.push(self.inner_fk_equal(self_tablealias));
        }

        // exists inside this exist clause
        for ins in &self.inner_exists_ins {
            if let Some(more) = ins.clause(&self.inner_tablealias, next_params) {
                inner_wheres.push(more);
            }
        }

        let inner_clauses = inner_wheres.join(" AND ");
        if using_in {
            Some(self.in_clause(self_tablealias, &inner_clauses))
        } else {
            Some(self.exists_clause(self_tablealias, &inner_clauses))
        }
    }
}
