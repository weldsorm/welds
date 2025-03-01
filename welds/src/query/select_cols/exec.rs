use crate::errors::Result;
use crate::model_traits::{HasSchema, TableColumns, TableInfo};
use crate::query::clause::ParamArgs;
use crate::query::helpers::{build_tail, build_where_clauses, join_sql_parts};
use crate::query::select_cols::SelectBuilder;
use crate::writers::ColumnWriter;
use crate::writers::NextParam;
use crate::Client;
use crate::Row;
use crate::Syntax;

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

        join_sql_parts(&[
            build_head_select(syntax, self),
            build_joins(syntax, self),
            where_sql,
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
        let syntax = client.syntax();
        let mut args: Option<ParamArgs> = Some(Vec::default());
        let sql = self.sql_internal(syntax, &mut args);
        let args = args.unwrap();
        let rows = client.fetch_rows(&sql, &args).await?;
        Ok(rows)
    }
}

fn build_head_select<T>(syntax: Syntax, sb: &SelectBuilder<T>) -> Option<String>
where
    T: HasSchema,
    <T as HasSchema>::Schema: TableInfo + TableColumns,
{
    let writer = ColumnWriter::new(syntax);
    let mut head: Vec<&str> = Vec::default();
    head.push("SELECT");

    let mut cols: Vec<_> = Vec::default();
    let alias = &sb.qb.alias;

    // Add these columns
    for col in &sb.selects {
        let colname = writer.excape(&col.col_name);
        let fieldname = writer.excape(&col.field_name);
        if colname == fieldname {
            let col = format!("{}.{}", alias, colname);
            cols.push(col);
        } else {
            let col = format!("{}.{} AS {}", alias, colname, fieldname);
            cols.push(col);
        }
    }

    // Add columns from joins
    for join in &sb.joins {
        join.append_columns(syntax, &mut cols);
    }

    let cols_text = cols.join(", ");
    head.push(&cols_text);

    head.push("FROM");
    let tn = <T as HasSchema>::Schema::identifier().join(".");
    let identifier = format!("{} {}", tn, alias);
    head.push(&identifier);
    Some(head.join(" "))
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
