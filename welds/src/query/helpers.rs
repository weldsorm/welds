use super::builder::QueryBuilder;
use super::clause::ParamArgs;
use crate::model_traits::{HasSchema, TableColumns, TableInfo};
//use crate::query::clause::exists::ExistIn;
use crate::query::clause::ClauseAdder;
use crate::writers::NextParam;
use crate::Syntax;

pub(crate) fn join_sql_parts(parts: &[Option<String>]) -> String {
    // Join al the parts into
    let sql: Vec<&str> = parts
        .iter()
        .filter_map(|x| x.as_ref().map(|x| x.as_str()))
        .collect();
    let sql: String = sql.as_slice().join(" ");
    sql
}

pub(crate) fn build_where<'lam, 'args, 'p>(
    syntax: Syntax,
    next_params: &NextParam,
    alias: &str,
    wheres: &'lam [Box<dyn ClauseAdder>],
    args: &'args mut Option<ParamArgs<'p>>,
    //exist_ins: &[ExistIn<'schema, DB>],
) -> Option<String>
where
    'lam: 'p,
{
    //let where_sql = build_where_clauses(next_params, alias, args, wheres, exist_ins);
    let where_sql = build_where_clauses(syntax, next_params, alias, wheres, args);
    if where_sql.is_empty() {
        return None;
    }
    Some(format!("WHERE ( {} )", where_sql.join(" AND ")))
}

pub(crate) fn build_where_clauses<'lam, 'args, 'p>(
    syntax: Syntax,
    next_params: &NextParam,
    alias: &str,
    wheres: &'lam [Box<dyn ClauseAdder>],
    args: &'args mut Option<ParamArgs<'p>>,
    //exist_ins: &[ExistIn<'schema, DB>],
) -> Vec<String>
where
    'lam: 'p,
{
    let mut where_sql: Vec<String> = Vec::default();
    for clause in wheres {
        if let Some(args) = args {
            clause.bind(args);
        }
        if let Some(p) = clause.clause(syntax, alias, next_params) {
            where_sql.push(p);
        }
    }
    //for clause in exist_ins {
    //    if let Some(args) = args {
    //        clause.bind(args);
    //    }
    //    if let Some(p) = clause.clause(alias, next_params) {
    //        where_sql.push(p);
    //    }
    //}
    where_sql
}

pub(crate) fn build_tail<T>(syntax: Syntax, select: &QueryBuilder<T>) -> Option<String>
where
    T: HasSchema,
{
    super::tail::write(syntax, &select.limit, &select.offset, &select.orderby)
}
