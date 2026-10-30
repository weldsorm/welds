use super::ClauseAdder;
use crate::Syntax;
use crate::query::builder::QueryBuilder;
use crate::query::clause::OrderBy;
use crate::query::clause::ParamArgs;
use crate::relations::RelationshipWriter;
use crate::writers::ColumnWriter;
use crate::writers::NextParam;
use crate::writers::TableWriter;
use crate::writers::alias::TableAlias;
use std::sync::Arc;

/// Used to generated a SQL EXISTS OR IN clause for writing sub-queries
#[derive(Clone)]
pub struct ExistIn {
    // how the outer table is linked to the inner table
    ship: RelationshipWriter,
    inner_tablename: &'static [&'static str],
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
        inner_tablename: &'static [&'static str],
        ship: RelationshipWriter,
    ) -> Self {
        ExistIn {
            // sub-queries are written as `inner.col = outer.col`
            ship: ship.inner_on_left(),
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

    // Write the parts of the relationship that link the inner table to the outer table
    fn relationship_clauses(
        &self,
        syntax: Syntax,
        tablealias: &str,
        next_params: &NextParam,
        using_in: bool,
    ) -> Vec<String> {
        let outer = tablealias;
        let inner = &self.inner_tablealias;
        if using_in {
            // the first key pair is written as the IN, don't write it twice
            self.ship
                .to_sql_without_first_key_pair(syntax, outer, inner, next_params)
        } else {
            self.ship.to_sql(syntax, outer, inner, next_params)
        }
    }

    fn tails(&self, syntax: Syntax, tablealias: &str) -> String {
        use crate::query::tail;
        tail::write(syntax, &self.limit, &self.offset, &self.orderby, tablealias)
            .unwrap_or_default()
    }

    fn exists_clause(&self, syntax: Syntax, _tablealias: &str, inner_clauses: &str) -> String {
        let tails = self.tails(syntax, &self.inner_tablealias);
        let inner_tablename = TableWriter::new(syntax).write2(self.inner_tablename);
        let col_writer = ColumnWriter::new(syntax);
        let select_col = match self.ship.first_key_pair() {
            Some((_, inner_col)) => col_writer.excape(inner_col),
            None => "1".to_string(),
        };
        format!(
            "EXISTS ( SELECT {} FROM {} {} WHERE {} {})",
            select_col, inner_tablename, &self.inner_tablealias, inner_clauses, tails
        )
    }

    fn in_clause(
        &self,
        syntax: Syntax,
        tablealias: &str,
        inner_clauses: &str,
        (outer_column, inner_column): (&str, &str),
    ) -> String {
        let col_writer = ColumnWriter::new(syntax);
        let outcol = format!("{}.{}", tablealias, col_writer.excape(outer_column));
        let innercol = format!(
            "{}.{}",
            self.inner_tablealias,
            col_writer.excape(inner_column)
        );
        let inner_tablename = TableWriter::new(syntax).write2(self.inner_tablename);
        let tails = self.tails(syntax, &self.inner_tablealias);
        let mut wheres = "".to_string();
        if !inner_clauses.is_empty() {
            wheres = format!("WHERE {}", inner_clauses);
        }
        format!(
            " {} IN (SELECT {} FROM {} {} {} {}) ",
            outcol, innercol, inner_tablename, self.inner_tablealias, wheres, tails
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
        // must match the order the clauses are written in `clause`
        self.ship.bind(args);
        for w in &self.inner_exists_ins {
            w.bind(args);
        }
    }

    fn clause(&self, syntax: Syntax, alias: &str, next_params: &NextParam) -> Option<String> {
        // When limited, the sub-query is written as an IN so the limit applies to the whole
        // sub-query and not per row. An IN needs a column to column link, fallback to EXISTS
        let in_cols = match self.limit {
            Some(_) => self.ship.first_key_pair(),
            None => None,
        };
        let using_in = in_cols.is_some();
        let self_tablealias = alias;
        let mut inner_wheres: Vec<String> = self
            .wheres
            .iter()
            .filter_map(|w| w.clause(syntax, &self.inner_tablealias, next_params))
            .collect();

        inner_wheres.append(&mut self.relationship_clauses(
            syntax,
            self_tablealias,
            next_params,
            using_in,
        ));

        // exists inside this exist clause
        for ins in &self.inner_exists_ins {
            if let Some(more) = ins.clause(syntax, &self.inner_tablealias, next_params) {
                inner_wheres.push(more);
            }
        }

        let inner_clauses = inner_wheres.join(" AND ");
        if let Some(in_cols) = in_cols {
            Some(self.in_clause(syntax, self_tablealias, &inner_clauses, in_cols))
        } else {
            Some(self.exists_clause(syntax, self_tablealias, &inner_clauses))
        }
    }
}
