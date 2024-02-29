use super::super::{
    builder::QueryBuilder,
    clause::{wherein::WhereIn, ClauseAdder},
    helpers::{build_where, join_sql_parts},
};
use crate::errors::Result;
use crate::model_traits::UniqueIdentifier;
use crate::model_traits::{HasSchema, TableColumns, TableInfo};
use crate::query::clause::ParamArgs;
use crate::writers::ColumnWriter;
use crate::writers::CountWriter;
use crate::writers::LimitSkipWriter;
use crate::writers::NextParam;
use crate::Syntax;
use welds_connections::Client;

// ******************************************************************************************
// This file contains code on a Query builder to allow it to bulk delete
// ******************************************************************************************

impl<T> QueryBuilder<T>
where
    T: Send + Unpin + HasSchema,
{
    /// The SQL to delete a `DELETE FROM ... `
    ///
    /// return SQL to delete all the resulting rows from the database
    pub fn delete_sql(&self, syntax: Syntax) -> String
    where
        <T as HasSchema>::Schema: TableInfo + TableColumns + UniqueIdentifier,
    {
        // we are wrapping this query in a where in clause.
        // This is needed if the user has a limit
        let mut w_in = WhereIn::new(self);

        self.delete_sql_internal(syntax, &mut w_in, &mut None)
    }

    fn delete_sql_internal<'s, 'w, 'args, 'p>(
        &'s self,
        syntax: Syntax,
        w_in: &'w mut WhereIn<T>,
        args: &'args mut Option<ParamArgs<'p>>,
    ) -> String
    where
        'w: 'p,
        's: 'p,
        <T as HasSchema>::Schema: UniqueIdentifier + TableInfo + TableColumns,
    {
        let next_params = NextParam::new(syntax);

        // Note: for deletes we can't alias the FROM tablename
        let alias = <T as HasSchema>::Schema::identifier().join(".");

        join_sql_parts(&[
            build_head_delete::<<T as HasSchema>::Schema>(syntax),
            build_where_delete(syntax, &next_params, &alias, args, self, w_in),
        ])
    }

    /// Executes a `DELETE FROM ... `
    ///
    /// deletes all the resulting rows from the database
    pub async fn delete<'s, 'c>(&'s self, client: &'c dyn Client) -> Result<()>
    where
        <T as HasSchema>::Schema: UniqueIdentifier + TableInfo + TableColumns,
    {
        // we are wrapping this query in a where in clause.
        // This is needed if the user has a limit
        let w_in_q = self;
        let mut w_in = WhereIn::new(w_in_q);

        let syntax = client.syntax();
        let mut args: Option<ParamArgs> = Some(Vec::default());
        let sql = self.delete_sql_internal(syntax, &mut w_in, &mut args);
        let args: ParamArgs = args.unwrap();
        client.execute(&sql, &args).await?;
        Ok(())
    }
}

fn build_head_delete<S>(syntax: Syntax) -> Option<String>
where
    S: TableInfo + TableColumns,
{
    let identifier = S::identifier().join(".");
    Some(format!("DELETE FROM {}", identifier))
}

fn build_where_delete<'args, 'p, 'qb, 'w, T>(
    syntax: Syntax,
    next_params: &NextParam,
    alias: &str,
    args: &'args mut Option<ParamArgs<'p>>,
    qb: &'qb QueryBuilder<T>,
    w_in: &'w mut WhereIn<T>,
) -> Option<String>
where
    'qb: 'p,
    'w: 'p,
    T: HasSchema,
    <T as HasSchema>::Schema: UniqueIdentifier + TableInfo + TableColumns,
{
    // If we have a limit, we need to wrap the wheres in an IN clause to
    // we can limit the number of row to delete
    if qb.limit.is_none() {
        let wheres = qb.wheres.as_slice();
        let exists_in = qb.exist_ins.as_slice();
        return build_where(syntax, next_params, alias, wheres, args, exists_in);
    }

    let mut where_sql: Vec<String> = Vec::default();

    if let Some(args) = args {
        w_in.bind(args);
    }

    if let Some(p) = w_in.clause(syntax, alias, next_params) {
        where_sql.push(p);
    }
    if where_sql.is_empty() {
        return None;
    }
    Some(format!("WHERE ( {} )", where_sql.join(" AND ")))
}
