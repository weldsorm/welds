use super::builder::QueryBuilder;
use super::clause::ParamArgs;
use super::helpers::{build_tail, build_where, join_sql_parts};
use crate::errors::Result;
use crate::model_traits::{HasSchema, TableColumns, TableInfo};
use crate::state::DbState;
use crate::writers::column::ColumnWriter;
use crate::writers::count::CountWriter;
use crate::writers::nextparam::NextParam;
use crate::{Syntax, WeldsError};
use welds_connections::Client;
use welds_connections::Row;

//use anyhow::Result;
//use sqlx::database::HasArguments;
//use sqlx::IntoArguments;
//use sqlx::Row;

// ******************************************************************************************
// This file contains all the stuff added onto the Querybuilder to allow it to run SELECTs
// ******************************************************************************************

impl<T> QueryBuilder<T>
where
    T: Send + HasSchema,
{
    /// Returns the SQL to count all rows in the resulting query
    pub fn to_sql_count<'s>(&'s self, syntax: Syntax) -> String
    where
        <T as HasSchema>::Schema: TableInfo + TableColumns,
    {
        self.sql_internal_count(syntax, &mut None)
    }

    fn sql_internal_count<'s, 'args, 'p>(
        &'s self,
        syntax: Syntax,
        args: &'args mut Option<ParamArgs<'p>>,
    ) -> String
    where
        's: 'p,
        <T as HasSchema>::Schema: TableInfo + TableColumns,
    {
        let next_params = NextParam::new(syntax);
        let wheres = self.wheres.as_slice();
        //let exists_in = self.exist_ins.as_slice();
        let alias = &self.alias;
        join_sql_parts(&[
            build_head_count::<<T as HasSchema>::Schema>(alias, syntax),
            //build_where(&next_params, alias, args, wheres, exists_in),
            build_where(syntax, &next_params, alias, wheres, args),
            build_tail(syntax, self),
        ])
    }

    /// Executes a `select count(...) FROM ... `
    ///
    /// Counts the results of your query in the database.
    pub async fn count<'q, 'c, C>(&'q self, client: &'c C) -> Result<u64>
    where
        C: Client,
        'q: 'c,
        <T as HasSchema>::Schema: TableInfo + TableColumns,
    {
        let syntax = client.syntax();
        let mut args: Option<ParamArgs> = Some(Vec::default());
        let sql = self.sql_internal_count(syntax, &mut args);

        let args = args.unwrap();
        let rows = client.fetch_rows(&sql, &args).await?;
        let row = rows.first().ok_or(WeldsError::RowNowFound)?;
        let count: i64 = row.get_by_position(0)?;
        Ok(count as u64)
    }

    /// Get a copy of the SQL that will be executed when this query runs
    pub fn to_sql<'s>(&'s self, syntax: Syntax) -> String
    where
        <T as HasSchema>::Schema: TableInfo + TableColumns,
    {
        self.sql_internal(syntax, &mut None)
    }

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
        let wheres = self.wheres.as_slice();
        //let exists_in = self.exist_ins.as_slice();
        let alias = &self.alias;
        join_sql_parts(&[
            build_head_select::<<T as HasSchema>::Schema>(alias, syntax),
            build_where(syntax, &next_params, alias, wheres, args),
            //build_where(&next_params, alias, args, wheres, exists_in),
            build_tail(syntax, self),
        ])
    }

    /// Executes the query in the database returning the results
    pub async fn run<'q, 'c, C>(&'q self, client: &'c C) -> Result<Vec<DbState<T>>>
    where
        C: Client,
        'q: 'c,
        <T as HasSchema>::Schema: TableInfo + TableColumns,
        T: TryFrom<Row>,
        WeldsError: From<<T as TryFrom<Row>>::Error>,
    {
        let syntax = client.syntax();
        let mut args: Option<ParamArgs> = Some(Vec::default());
        let sql = self.sql_internal(syntax, &mut args);

        let args = args.unwrap();
        let rows = client.fetch_rows(&sql, &args).await?;

        let mut objs = Vec::default();
        for row in rows {
            let obj: T = T::try_from(row)?;
            objs.push(DbState::db_loaded(obj));
        }
        Ok(objs)
    }
}

fn build_head_select<S>(tablealias: &str, syntax: Syntax) -> Option<String>
where
    S: TableInfo + TableColumns,
{
    let writer = ColumnWriter::new(syntax);
    let mut head: Vec<&str> = Vec::default();
    head.push("SELECT");
    let cols_info = S::columns();
    let cols: Vec<_> = cols_info
        .iter()
        .map(|col| writer.write(tablealias, col))
        .collect();
    let cols = cols.join(", ");
    head.push(&cols);
    head.push("FROM");
    let tn = S::identifier().join(".");
    let identifier = format!("{} {}", tn, tablealias);
    head.push(&identifier);
    Some(head.join(" "))
}

fn build_head_count<S>(tablealias: &str, syntax: Syntax) -> Option<String>
where
    S: TableInfo + TableColumns,
{
    let tn = S::identifier().join(".");
    let identifier = format!("{} {}", tn, &tablealias);
    let cw = CountWriter::new(syntax);
    let count_star = cw.count(Some(tablealias), Some("*"));
    Some(format!("SELECT {} FROM {}", count_star, identifier))
}

#[cfg(test)]
mod tests;
