use super::{ClauseContext, KeyPair, Relationship, RelationshipClause};
use crate::Syntax;
use crate::query::clause::ParamArgs;
use crate::writers::NextParam;
use std::sync::Arc;

/// Which table in the generated SQL is the table the relationship is defined on ("my" table)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Side {
    /// The outer query / left side of a join is the table the relationship was defined on
    MineIsOuter,
    /// The inner query / right side of a join is the table the relationship was defined on
    MineIsInner,
}

/// Captures the clauses of a relationship so they can be written as SQL
/// in a JOIN ON or as the correlated part of a sub-query.
#[derive(Clone)]
pub(crate) struct RelationshipWriter {
    clauses: Vec<Arc<dyn RelationshipClause>>,
    side: Side,
    outer_on_left: bool,
}

impl RelationshipWriter {
    pub(crate) fn new<T, R>(ship: &impl Relationship<T, R>, side: Side) -> Self {
        let clauses = ship.clauses().into_iter().map(Arc::from).collect();
        Self {
            clauses,
            side,
            outer_on_left: true,
        }
    }

    /// Write column comparisons with the inner table on the left. `inner.col = outer.col`
    /// This is how sub-queries have always been written
    pub(crate) fn inner_on_left(mut self) -> Self {
        self.outer_on_left = false;
        self
    }

    fn aliases<'a>(&self, outer_alias: &'a str, inner_alias: &'a str) -> (&'a str, &'a str) {
        match self.side {
            Side::MineIsOuter => (outer_alias, inner_alias),
            Side::MineIsInner => (inner_alias, outer_alias),
        }
    }

    /// The first column to column clause. Returned as (outer_column, inner_column)
    pub(crate) fn first_key_pair(&self) -> Option<(&str, &str)> {
        let kp: &KeyPair = self.clauses.iter().find_map(|c| c.as_key_pair())?;
        Some(match self.side {
            Side::MineIsOuter => (kp.my_key.as_str(), kp.their_key.as_str()),
            Side::MineIsInner => (kp.their_key.as_str(), kp.my_key.as_str()),
        })
    }

    /// returns the index of the clause that is the first key pair
    fn first_key_pair_index(&self) -> Option<usize> {
        self.clauses.iter().position(|c| c.as_key_pair().is_some())
    }

    /// Writes all the clauses of the relationship ANDed together
    pub(crate) fn to_sql(
        &self,
        syntax: Syntax,
        outer_alias: &str,
        inner_alias: &str,
        next_params: &NextParam,
    ) -> Vec<String> {
        self.to_sql_skipping(syntax, outer_alias, inner_alias, next_params, None)
    }

    /// Writes all the clauses except the first key pair.
    /// Used when the first key pair is written as `outer.col IN (SELECT inner.col ...)`
    pub(crate) fn to_sql_without_first_key_pair(
        &self,
        syntax: Syntax,
        outer_alias: &str,
        inner_alias: &str,
        next_params: &NextParam,
    ) -> Vec<String> {
        let skip = self.first_key_pair_index();
        self.to_sql_skipping(syntax, outer_alias, inner_alias, next_params, skip)
    }

    fn to_sql_skipping(
        &self,
        syntax: Syntax,
        outer_alias: &str,
        inner_alias: &str,
        next_params: &NextParam,
        skip: Option<usize>,
    ) -> Vec<String> {
        let (my_alias, their_alias) = self.aliases(outer_alias, inner_alias);
        let mine_is_outer = self.side == Side::MineIsOuter;
        let ctx = ClauseContext {
            syntax,
            my_alias,
            their_alias,
            next_params,
            mine_on_left: mine_is_outer == self.outer_on_left,
        };
        self.clauses
            .iter()
            .enumerate()
            .filter(|(i, _)| Some(*i) != skip)
            .map(|(_, c)| c.write_sql(&ctx))
            .collect()
    }

    /// Adds the args used by the clauses in the same order they are written.
    /// KeyPairs have no args, so skipping one when writing never changes the arg order
    pub(crate) fn bind<'lam, 'args, 'p>(&'lam self, args: &'args mut ParamArgs<'p>)
    where
        'lam: 'p,
    {
        for c in &self.clauses {
            c.bind(args);
        }
    }
}
