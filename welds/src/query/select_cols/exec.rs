use crate::Row;
use crate::Syntax;
use crate::errors::Result;
use crate::model_traits::{HasSchema, TableColumns, TableInfo};
use crate::query::clause::ParamArgs;
use crate::query::helpers::{build_tail, build_where_clauses, join_sql_parts};
use crate::query::select_cols::SelectBuilder;
use crate::query::select_cols::select_column::SelectRender;
use crate::writers::TableWriter;
use crate::writers::{ColumnWriter, NextParam};
use crate::{Client, WeldsError};
use std::collections::HashSet;
use welds_connections::trace;

// ******************************************************************************************
// This file contains all the stuff added onto the SelectBuilder to allow it to run SELECTs
// ******************************************************************************************

impl<T> SelectBuilder<T>
where
    T: Send + HasSchema,
{
    fn sql_internal<'s, 'args, 'p>(
        &'s self,
        syntax: Syntax,
        args: &'args mut Option<ParamArgs<'p>>,
    ) -> String
    where
        's: 'p,
        <T as HasSchema>::Schema: TableInfo + TableColumns,
    {
        let next_params = NextParam::new(syntax);
        let wheres = self.qb.wheres.as_slice();
        let exists_in = self.qb.exist_ins.as_slice();
        let alias = &self.qb.alias;

        let mut wheres = build_where_clauses(syntax, &next_params, alias, wheres, args, exists_in);
        for j in &self.joins {
            j.append_where(syntax, &mut wheres, &next_params, args);
        }
        let where_sql = if wheres.is_empty() {
            None
        } else {
            Some(format!("WHERE ( {} )", wheres.join(" AND ")))
        };

        let select_renders = build_select_renders(self);

        join_sql_parts(&[
            build_head_select(syntax, &select_renders, self),
            build_joins(syntax, self),
            where_sql,
            build_group_by(syntax, &select_renders, self),
            build_tail(syntax, &self.qb),
        ])
        .trim()
        .to_owned()
    }

    /// Get a copy of the SQL that will be executed when this query runs
    pub fn to_sql(&self, syntax: Syntax) -> String
    where
        <T as HasSchema>::Schema: TableInfo + TableColumns,
    {
        self.sql_internal(syntax, &mut None)
    }

    /// Executes the query in the database returning the results
    pub async fn run(&self, client: &dyn Client) -> Result<Vec<Row>>
    where
        <T as HasSchema>::Schema: TableInfo + TableColumns,
    {
        trace::db_error(self.validate_group_by())?;
        let syntax = client.syntax();
        let mut args: Option<ParamArgs> = Some(Vec::default());
        let sql = self.sql_internal(syntax, &mut args);
        let args = args.unwrap();
        let rows = client.fetch_rows(&sql, &args).await?;
        Ok(rows)
    }

    fn validate_group_by(&self) -> Result<()> {
        if self.requires_group_by() && self.group_bys.is_empty() {
            return Err(WeldsError::ColumnMissingFromGroupBy);
        }
        Ok(())
    }

    fn requires_group_by(&self) -> bool {
        self.selects.iter().any(|s| s.is_aggregate())
            && self.selects.iter().any(|s| !s.is_aggregate())
    }
}

/// write the head of of the select statement
fn build_head_select<T>(
    syntax: Syntax,
    columns: &[SelectRender],
    sb: &SelectBuilder<T>,
) -> Option<String>
where
    T: HasSchema,
    <T as HasSchema>::Schema: TableInfo + TableColumns,
{
    let mut head: Vec<&str> = Vec::default();
    head.push("SELECT");

    let mut cols_text_parts: Vec<_> = Vec::default();
    for col in columns {
        cols_text_parts.push(col.write(syntax))
    }

    let cols_text = cols_text_parts.join(", ");
    head.push(&cols_text);

    head.push("FROM");

    let parts = <T as HasSchema>::Schema::identifier();
    let tn = TableWriter::new(syntax).write2(parts);

    let alias = &sb.qb.alias;
    let identifier = format!("{} {}", tn, alias);
    head.push(&identifier);
    Some(head.join(" "))
}

/// Gather all the info needed to render each of the columns in the select
fn build_select_renders<T>(sb: &SelectBuilder<T>) -> Vec<SelectRender>
where
    T: HasSchema,
    <T as HasSchema>::Schema: TableInfo + TableColumns,
{
    let mut parts: Vec<SelectRender> = Vec::default();

    // Add these columns
    let alias = &sb.qb.alias;
    for select in &sb.selects {
        parts.push(SelectRender::new(select, alias));
    }

    // Add columns from joins
    for join in &sb.joins {
        join.append_select_renders(&mut parts);
    }

    parts
}

fn build_joins<T>(syntax: Syntax, sb: &SelectBuilder<T>) -> Option<String>
where
    T: HasSchema,
    <T as HasSchema>::Schema: TableInfo + TableColumns,
{
    let mut list = Vec::default();
    let alias = &sb.qb.alias;
    // Add columns from joins
    for join in &sb.joins {
        join.append_jointable(syntax, &mut list, alias);
    }
    Some(list.join(" "))
}

fn build_group_by<T>(
    syntax: Syntax,
    columns: &[SelectRender],
    sb: &SelectBuilder<T>,
) -> Option<String>
where
    T: HasSchema,
    <T as HasSchema>::Schema: TableInfo + TableColumns,
{
    // if we aren't grouping by anything, abort
    if sb.group_bys.is_empty() {
        return None;
    }

    let writer = ColumnWriter::new(syntax);
    let mut cols: Vec<String> = Vec::default();

    // build a list of all the columns that are expected to be in a grouped by
    let mut must_group: HashSet<(&str, &str)> = columns
        .iter()
        .filter(|x| !x.is_aggregate())
        .map(|x| (x.alias.as_str(), x.col_name.as_str()))
        .collect();

    for group_by in &sb.group_bys {
        let alias = group_by.table_alias.as_ref().unwrap_or(&sb.qb.alias);
        cols.push(format!("{}.{}", alias, writer.excape(&group_by.col_name)));
        must_group.remove(&(alias.as_str(), group_by.col_name.as_str()));
    }

    // We have detected column that the user has selected, that *should* be grouped by.
    // This isn't an error in all databases, but could be an ambiguous query
    //
    //if !must_group.is_empty() {
    //    let columns: Vec<_> = must_group.iter().collect();
    //    log::warn!("group by missing columns {:?}", columns);
    //}

    Some(format!("GROUP BY {}", cols.join(", ")))
}
