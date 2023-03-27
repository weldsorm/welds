use super::ClauseAdder;
use crate::query::select::SelectBuilder;
use std::marker::PhantomData;

/// Used to generated a SQL EXISTS clause for writing sub-queries

pub struct ExistIn<'args, DB> {
    outer_column: String,
    outer_tablealias: String,
    inner_column: String,
    inner_tablename: String,
    inner_tablealias: String,
    wheres: Vec<Box<dyn ClauseAdder<'args, DB>>>,
    inner_exists_ins: Vec<Self>,
}

impl<'args, DB> ExistIn<'args, DB>
where
    DB: sqlx::Database,
{
    pub(crate) fn new<T>(
        sb: SelectBuilder<'args, T, DB>,
        outer_tablealias: String,
        outer_column: String,
        inner_tablename: String,
        inner_tablealias: String,
        inner_column: String,
    ) -> Self {
        ExistIn::<'args, DB> {
            outer_column,
            outer_tablealias,
            inner_column,
            inner_tablename,
            inner_tablealias,
            wheres: sb.wheres, //wheres: sb.wheres,
            inner_exists_ins: sb.exist_ins,
        }
    }

    fn inner_fk_equal(&self) -> String {
        format!(
            "{}.{} = {}.{}",
            self.inner_tablealias, self.inner_column, self.outer_tablealias, self.outer_column
        )
    }

    fn exists_clause(&self, inner_clauses: &str) -> String {
        format!(
            "EXISTS ( SELECT {} FROM {} {} WHERE {} )",
            self.inner_column, self.inner_tablename, self.inner_tablealias, inner_clauses
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

    fn clause(&self, next_params: &super::NextParam) -> Option<String> {
        let mut inner_wheres: Vec<String> = self
            .wheres
            .iter()
            .map(|w| w.clause(next_params))
            .filter_map(|x| x)
            .collect();
        inner_wheres.push(self.inner_fk_equal());

        // exists inside this exist clause
        for ins in &self.inner_exists_ins {
            if let Some(more) = ins.clause(next_params) {
                inner_wheres.push(more);
            }
        }

        let inner_clauses = inner_wheres.join(" AND ");
        Some(self.exists_clause(&inner_clauses))
    }

    fn set_tablealias(&mut self, _alias: String) {
        panic!("not used");
    }
}
