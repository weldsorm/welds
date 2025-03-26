use crate::query::clause::orderby;
use crate::query::clause::OrderBy;
use crate::writers::limit_skip::LimitSkipWriter;
use crate::Syntax;
use std::collections::VecDeque;

/// writes the Limit Skip OrderBy of a statement
pub(crate) fn write(
    syntax: Syntax,
    limit: &Option<i64>,
    offset: &Option<i64>,
    orders: &[OrderBy],
    table_alias: &str,
) -> Option<String> {
    let w = LimitSkipWriter::new(syntax);
    let mut parts = VecDeque::default();

    if let Some(skiplimit) = w.skiplimit(offset, limit) {
        parts.push_back(skiplimit);
    }

    // If we are limiting but no order is given force an order (needed for MSSQL)
    if !parts.is_empty() && orders.is_empty() {
        parts.push_front("ORDER BY 1".to_owned())
    }

    if !orders.is_empty() {
        parts.push_front(orderby::to_sql(orders, table_alias));
    }

    if parts.is_empty() {
        return None;
    }
    let parts: Vec<String> = parts.drain(..).collect();
    Some(parts.join(" "))
}
