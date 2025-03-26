use crate::dataset::DataSet;
use crate::errors::Result;
use crate::errors::WeldsError;
use crate::model_traits::{HasSchema, TableColumns, TableInfo};
use crate::query::include::IncludeBuilder;
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
    /// Executes the query in the database returning the results
    pub fn to_sql<'q, 'c>(&'q self, syntax: Syntax) -> Vec<String>
    where
        'q: 'c,
        <T as HasSchema>::Schema: TableInfo + TableColumns,
        T: TryFrom<Row>,
        WeldsError: From<<T as TryFrom<Row>>::Error>,
    {
        let mut sqls: Vec<_> = vec![self.qb.to_sql(syntax)];
        for related_query in &self.related {
            sqls.push(related_query.to_sql(&self.qb, syntax));
        }
        sqls
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

        let mut related = Vec::default();
        for related_query in &self.related {
            related.push(related_query.run(&self.qb, client).await?);
        }

        Ok(DataSet::new(primary, related))
    }
}
