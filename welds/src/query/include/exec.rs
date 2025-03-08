use crate::dataset::DataSet;
use crate::errors::Result;
use crate::errors::WeldsError;
use crate::model_traits::{HasSchema, TableColumns, TableInfo};
use crate::query::clause::ParamArgs;
use crate::query::helpers::{build_tail, build_where_clauses, join_sql_parts};
use crate::query::include::IncludeBuilder;
use crate::writers::ColumnWriter;
use crate::writers::NextParam;
use crate::Client;
use crate::Row;
use crate::Syntax;

// ******************************************************************************************
// This file contains all the stuff added onto the IncludeBuilder to allow it to run Execute the Query
// ******************************************************************************************

impl<T> IncludeBuilder<T>
where
    T: Send + HasSchema,
{
    /// Get a copy of the SQL that will be executed when this query runs
    pub fn to_sql(&self, syntax: Syntax) -> Vec<String>
    where
        <T as HasSchema>::Schema: TableInfo + TableColumns,
    {
        todo!()
    }

    /// Executes the query in the database returning the results
    pub async fn run<'q, 'c>(&'q self, client: &'c dyn Client) -> Result<DataSet<T>>
    where
        'q: 'c,
        <T as HasSchema>::Schema: TableInfo + TableColumns,
        T: TryFrom<Row>,
        WeldsError: From<<T as TryFrom<Row>>::Error>,
    {
        let primary = self.qb.run(client).await?;

        // Don't know how we are going to include the related queries until DataSet knowns how it
        // wan't them
        // let base2 = self.qb.run(client).await?;
        // let base3 = self.qb.run(client).await?;

        todo!()
        Ok(DataSet::new(primary))
    }
}
