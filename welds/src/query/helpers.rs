use super::builder::QueryBuilder;
use super::clause::NextParam;
use crate::alias;
use crate::query::clause::exists::ExistIn;
use crate::query::clause::ClauseAdder;
use crate::table::{HasSchema, TableColumns, TableInfo};
use crate::writers::limit_skip::DbLimitSkipWriter;
use sqlx::database::HasArguments;
use sqlx::IntoArguments;

pub(crate) fn join_sql_parts(parts: &[Option<String>]) -> String {
    // Join al the parts into
    let sql: Vec<&str> = parts
        .iter()
        .filter_map(|x| x.as_ref().map(|x| x.as_str()))
        .collect();
    let sql: String = sql.as_slice().join(" ");
    sql
}

pub(crate) fn build_where<'schema, 'args, DB>(
    next_params: &NextParam,
    alias: &str,
    args: &mut Option<<DB as HasArguments<'schema>>::Arguments>,
    wheres: &[Box<dyn ClauseAdder<'schema, DB>>],
    exist_ins: &[ExistIn<'schema, DB>],
) -> Option<String>
where
    DB: sqlx::Database + DbLimitSkipWriter,
    <DB as HasArguments<'schema>>::Arguments: IntoArguments<'args, DB>,
{
    let where_sql = build_where_clauses(next_params, alias, args, wheres, exist_ins);
    if where_sql.is_empty() {
        return None;
    }
    Some(format!("WHERE ( {} )", where_sql.join(" AND ")))
}

pub(crate) fn build_where_clauses<'schema, 'args, DB>(
    next_params: &NextParam,
    alias: &str,
    args: &mut Option<<DB as HasArguments<'schema>>::Arguments>,
    wheres: &[Box<dyn ClauseAdder<'schema, DB>>],
    exist_ins: &[ExistIn<'schema, DB>],
) -> Vec<String>
where
    DB: sqlx::Database + DbLimitSkipWriter,
    <DB as HasArguments<'schema>>::Arguments: IntoArguments<'args, DB>,
{
    let mut where_sql: Vec<String> = Vec::default();
    for clause in wheres {
        if let Some(args) = args {
            clause.bind(args);
        }
        if let Some(p) = clause.clause(alias, next_params) {
            where_sql.push(p);
        }
    }
    for clause in exist_ins {
        if let Some(args) = args {
            clause.bind(args);
        }
        if let Some(p) = clause.clause(alias, next_params) {
            where_sql.push(p);
        }
    }
    where_sql
}

pub(crate) fn build_tail<T, DB>(select: &QueryBuilder<T, DB>) -> Option<String>
where
    T: HasSchema,
    DB: sqlx::Database + DbLimitSkipWriter,
    <T as HasSchema>::Schema: TableInfo + TableColumns<DB>,
{
    super::tail::write::<DB>(&select.limit, &select.offset, &select.orderby)
}
