use super::ClauseAdder;
use crate::query::builder::QueryBuilder;
use crate::query::clause::OrderBy;
use crate::query::clause::ParamArgs;
use crate::writers::alias::TableAlias;
use crate::writers::NextParam;
use crate::Syntax;
use std::sync::Arc;

/// Used to generated a SQL EXISTS OR IN clause for writing sub-queries
#[derive(Clone)]
pub struct ExistIn {
    outer_column: String,
    inner_column: String,
    inner_tablename: String,
    pub(crate) inner_tablealias: String,
    wheres: Vec<Arc<Box<dyn ClauseAdder>>>,
    inner_exists_ins: Vec<Self>,
    limit: Option<i64>,
    offset: Option<i64>,
    orderby: Vec<OrderBy>,
}

impl ExistIn {
    pub(crate) fn new<T>(
        sb: &QueryBuilder<T>,
        outer_column: String,
        inner_tablename: String,
        inner_column: String,
    ) -> Self {
        ExistIn {
            outer_column,
            inner_column,
            inner_tablename,
            inner_tablealias: sb.alias.clone(),
            wheres: sb.wheres.clone(),
            inner_exists_ins: sb.exist_ins.clone(),
            limit: sb.limit,
            offset: sb.offset,
            orderby: sb.orderby.clone(),
        }
    }

    // re-assign all the alias and alias for sub-tables
    pub(crate) fn set_aliases(&mut self, alias_asigner: &Arc<TableAlias>) {
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

    fn tails(&self, syntax: Syntax, tablealias: &str) -> String {
        use crate::query::tail;
        tail::write(syntax, &self.limit, &self.offset, &self.orderby, tablealias)
            .unwrap_or_default()
    }

    fn exists_clause(&self, syntax: Syntax, _tablealias: &str, inner_clauses: &str) -> String {
        let tails = self.tails(syntax, &self.inner_tablealias);
        format!(
            "EXISTS ( SELECT {} FROM {} {} WHERE {} {})",
            self.inner_column, self.inner_tablename, self.inner_tablealias, inner_clauses, tails
        )
    }

    fn in_clause(&self, syntax: Syntax, tablealias: &str, inner_clauses: &str) -> String {
        let outcol = format!("{}.{}", tablealias, self.outer_column);
        let innercol = format!("{}.{}", self.inner_tablealias, self.inner_column);
        let tails = self.tails(syntax, &self.inner_tablealias);
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

impl ClauseAdder for ExistIn {
    fn bind<'lam, 'args, 'p>(&'lam self, args: &'args mut ParamArgs<'p>)
    where
        'lam: 'p,
    {
        for w in &self.wheres {
            w.bind(args);
        }
        for w in &self.inner_exists_ins {
            w.bind(args);
        }
    }

    fn clause(&self, syntax: Syntax, alias: &str, next_params: &NextParam) -> Option<String> {
        let using_in = self.limit.is_some();
        let self_tablealias = alias;
        let mut inner_wheres: Vec<String> = self
            .wheres
            .iter()
            .filter_map(|w| w.clause(syntax, &self.inner_tablealias, next_params))
            .collect();

        if !using_in {
            inner_wheres.push(self.inner_fk_equal(self_tablealias));
        }

        // exists inside this exist clause
        for ins in &self.inner_exists_ins {
            if let Some(more) = ins.clause(syntax, &self.inner_tablealias, next_params) {
                inner_wheres.push(more);
            }
        }

        let inner_clauses = inner_wheres.join(" AND ");
        if using_in {
            Some(self.in_clause(syntax, self_tablealias, &inner_clauses))
        } else {
            Some(self.exists_clause(syntax, self_tablealias, &inner_clauses))
        }
    }
}
