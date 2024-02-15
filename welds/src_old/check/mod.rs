use crate::connection::Connection;
use crate::connection::DbProvider;
use crate::detect::ColumnDef;
use crate::detect::TableScan;
use crate::errors::Result;
use crate::table::Column;
use crate::table::{HasSchema, TableColumns, TableInfo};
use sqlx::database::HasArguments;
use sqlx::IntoArguments;

mod issue;
pub use issue::*;

/// Returns a list of differences in the current database schema
/// and what the welds object was compiled against
///
/// Used to known if there are going to be issues when running the query of a model
pub async fn schema<'c, 'args, T, DB, C>(conn: &'c C) -> Result<Vec<Issue>>
where
    'c: 'args,
    C: Connection<DB>,
    <DB as HasArguments<'args>>::Arguments: IntoArguments<'args, DB>,
    T: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row> + HasSchema,
    <T as HasSchema>::Schema: TableInfo + TableColumns<DB>,
    DB: crate::connection::Database + TableScan,
    usize: sqlx::ColumnIndex<<DB as sqlx::Database>::Row>,
    i32: sqlx::Type<DB> + for<'r> sqlx::Decode<'r, DB>,
    String: sqlx::Type<DB> + for<'r> sqlx::Decode<'r, DB>,
    for<'x> &'x str: sqlx::Type<DB> + sqlx::Encode<'x, DB>,
    for<'x> Option<&'x str>: sqlx::Type<DB> + sqlx::Encode<'x, DB>,
{
    let mut problems = Vec::default();
    let identifier_parts: Vec<&str> = <T::Schema>::identifier().iter().rev().cloned().collect();
    let tablename = identifier_parts[0];
    let namespace = identifier_parts.get(1).copied();
    let namespace = unwrap_to_default_namespace(namespace, conn);

    let tabledef = match crate::detect::find_table(namespace, tablename, conn).await? {
        Some(x) => x,
        None => return Ok(vec![Issue::missing_table(namespace, tablename)]),
    };
    let table_cols = tabledef.columns();
    let model_cols = <T::Schema>::columns();

    struct_added(table_cols, &model_cols)
        .iter()
        .for_each(|x| problems.push(Issue::struct_added(namespace, tablename, x)));

    changed(table_cols, &model_cols)
        .iter()
        .for_each(|x| problems.push(Issue::changed(namespace, tablename, x)));

    struct_missing(table_cols, &model_cols)
        .iter()
        .for_each(|x| problems.push(Issue::struct_missing(namespace, tablename, x)));

    Ok(problems)
}

/// returns the default namespace that is used if no namespace is provided
fn unwrap_to_default_namespace<C, DB>(ns: Option<&'static str>, conn: &C) -> Option<&'static str>
where
    C: Connection<DB>,
    DB: crate::connection::Database,
{
    if ns.is_some() {
        return ns;
    }
    match conn.provider() {
        DbProvider::Mssql => Some("dbo"),
        DbProvider::Postgres => Some("public"),
        // NOTE if schema is left out, the mysql query uses the name of the db in the connection
        DbProvider::MySql => None,
        DbProvider::Sqlite => None,
    }
}

fn struct_missing<'a>(table_cols: &'a [ColumnDef], model_cols: &[Column]) -> Vec<&'a ColumnDef> {
    let model_has = |name: &str| model_cols.iter().any(|x| x.name() == name);
    table_cols
        .iter()
        .filter(|tc| !model_has(&tc.name))
        .collect()
}

fn struct_added<'a>(table_cols: &[ColumnDef], model_cols: &'a [Column]) -> Vec<&'a Column> {
    let table_has = |name: &str| table_cols.iter().any(|x| x.name == name);
    model_cols
        .iter()
        .filter(|mc| !table_has(mc.name()))
        .collect()
}

fn changed<'a>(
    table_cols: &'a [ColumnDef],
    model_cols: &'a [Column],
) -> Vec<(&'a ColumnDef, &'a Column)> {
    let table_find = |name: &str| table_cols.iter().find(|x| x.name == name);
    model_cols
        .iter()
        .map(|mc| (table_find(mc.name()), mc))
        .filter(|x| x.0.is_some())
        .map(|x| (x.0.unwrap(), x.1))
        .filter(|x| !same_types(&x.0.ty, x.1.dbtype()) || x.0.null != x.1.nullable())
        .collect()
}

/// Returns true if the two types are compatible
/// same_types("INT4", "INT4") == true
fn same_types(t1: &str, t2: &str) -> bool {
    if t1 == t2 {
        return true;
    }
    if let Some(group) = find_same_group(t1) {
        for x in group {
            if t2 == *x {
                return true;
            }
        }
    }
    false
}

fn find_same_group(t: &str) -> Option<&'static [&'static str]> {
    for group in SAME_TYPES {
        for inner in group.iter() {
            if *inner == t {
                return Some(group);
            }
        }
    }
    None
}

// list of all types that are compatible with each other.
const SAME_TYPES: &[&[&str]] = &[
    &["TEXT", "VARCHAR", "NVARCHAR"],
    &["INT4", "INT", "SERIAL", "BIT", "NBIT"],
    &["BIGINT", "INT8", "BIGSERIAL"],
    &["BINYINT", "BOOLEAN"],
    &["TINYINT", "BOOLEAN"],
];
