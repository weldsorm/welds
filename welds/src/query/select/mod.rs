use super::builder::QueryBuilder;
use super::clause::ParamArgs;
use crate::errors::Result;
use crate::model_traits::{HasSchema, TableColumns, TableIdent, TableInfo};
use crate::state::DbState;
use crate::{Syntax, WeldsError};
use welds_connections::Client;
use welds_connections::Row;

mod writer;
pub use writer::SelectWriter;

// ******************************************************************************************
// This file contains all the stuff added onto the Querybuilder to allow it to run SELECTs
// ******************************************************************************************

impl<T> QueryBuilder<T>
where
    T: Send + HasSchema,
{
    /// Returns the SQL to count all rows in the resulting query
    pub fn to_sql_count(&self, syntax: Syntax) -> String
    where
        T: HasSchema,
        <T as HasSchema>::Schema: TableInfo + TableColumns,
    {
        let table = TableIdent::from_model::<T>();
        let writer = SelectWriter::new_with_alias(syntax, &table, &self.alias);
        writer.sql_count(
            &self.wheres,
            &self.exist_ins,
            &self.limit,
            &self.offset,
            &self.orderby,
            &mut None,
        )
    }

    /// Executes a `select count(...) FROM ... `
    ///
    /// Counts the results of your query in the database.
    pub async fn count<'q, 'c>(&'q self, client: &'c dyn Client) -> Result<u64>
    where
        'q: 'c,
        <T as HasSchema>::Schema: TableInfo + TableColumns,
    {
        let syntax = client.syntax();
        let mut args: Option<ParamArgs> = Some(Vec::default());

        let table = TableIdent::from_model::<T>();
        let writer = SelectWriter::new_with_alias(syntax, &table, &self.alias);
        let sql = writer.sql_count(
            &self.wheres,
            &self.exist_ins,
            &self.limit,
            &self.offset,
            &self.orderby,
            &mut args,
        );

        let args = args.unwrap();
        let rows = client.fetch_rows(&sql, &args).await?;
        let row = rows.first().ok_or(WeldsError::RowNowFound)?;
        let count: i64 = row.get_by_position(0)?;
        Ok(count as u64)
    }

    /// Get a copy of the SQL that will be executed when this query runs
    pub fn to_sql(&self, syntax: Syntax) -> String
    where
        <T as HasSchema>::Schema: TableInfo + TableColumns,
    {
        let table = TableIdent::from_model::<T>();
        let columns = <T as HasSchema>::Schema::columns();
        let writer = SelectWriter::new_with_alias(syntax, &table, &self.alias);
        writer.sql(
            &columns,
            &self.wheres,
            &self.exist_ins,
            &self.limit,
            &self.offset,
            &self.orderby,
            &mut None,
        )
    }

    /// Executes the query in the database returning the results
    pub async fn run<'q, 'c>(&'q self, client: &'c dyn Client) -> Result<Vec<DbState<T>>>
    where
        'q: 'c,
        <T as HasSchema>::Schema: TableInfo + TableColumns,
        T: TryFrom<Row>,
        WeldsError: From<<T as TryFrom<Row>>::Error>,
    {
        let syntax = client.syntax();
        let mut args: Option<ParamArgs> = Some(Vec::default());

        let table = TableIdent::from_model::<T>();
        let columns = <T as HasSchema>::Schema::columns();
        let writer = SelectWriter::new_with_alias(syntax, &table, &self.alias);
        let sql = writer.sql(
            &columns,
            &self.wheres,
            &self.exist_ins,
            &self.limit,
            &self.offset,
            &self.orderby,
            &mut args,
        );

        let args = args.unwrap();
        let rows = client.fetch_rows(&sql, &args).await?;

        let mut objs = Vec::default();
        for row in rows {
            let obj: T = T::try_from(row)?;
            objs.push(DbState::db_loaded(obj));
        }
        Ok(objs)
    }

    pub async fn fetch_one<'q, 'c>(&self, client: &'c dyn Client) -> Result<DbState<T>>
    where
        'q: 'c,
        <T as HasSchema>::Schema: TableInfo + TableColumns,
        T: TryFrom<Row>,
        WeldsError: From<<T as TryFrom<Row>>::Error>,
    {
        let mut query = self.clone();
        query.limit = Some(1);
        query.run(client).await?.into_iter().nth(0).ok_or(WeldsError::RowNowFound)
    }
}

#[cfg(test)]
mod tests;
