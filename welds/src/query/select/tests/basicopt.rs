use super::*;
use crate::model_traits::Column;
use crate::model_traits::HasSchema;
use crate::model_traits::TableColumns;
use crate::model_traits::TableInfo;
use crate::query::clause::BasicOpt;

// Test Object that can be used to write SQL
// Testing with null/some/and unwrapped values
//

// model with option

struct Product2 {
    pub name: Option<String>,
}

impl TryFrom<Row> for Product2 {
    type Error = crate::WeldsError;
    fn try_from(_value: Row) -> std::result::Result<Self, Self::Error> {
        Ok(Product2 { name: None })
    }
}

pub struct Product2Schema {
    name: BasicOpt<String>,
}

impl Default for Product2Schema {
    fn default() -> Self {
        Self {
            name: BasicOpt::new("dbname", "name"),
        }
    }
}

impl TableInfo for Product2Schema {
    fn identifier() -> &'static [&'static str] {
        &["da_schemaname", "da_tablename"]
    }
}

impl TableColumns for Product2Schema {
    type ColumnStruct = Self;
    fn columns() -> Vec<Column> {
        vec![Column::new("dbname", "String", true)]
    }
    fn primary_keys() -> Vec<Column> {
        vec![]
    }
}

impl HasSchema for Product2 {
    type Schema = Product2Schema;
}

// Tests

#[test]
fn should_exec_basicopt_with_where_with_value() {
    futures::executor::block_on(async move {
        let q = QueryBuilder::<Product2>::new().where_col(|c| c.name.equal("bla"));
        let client = welds_connections::noop::build(Syntax::Mssql);
        q.run(&client).await.unwrap();
        let ran_sql = client.last_sql().unwrap();
        let expected =
            "SELECT t1.\"dbname\" FROM da_schemaname.da_tablename t1 WHERE ( t1.dbname = @p1 )";
        assert_eq!(expected, &ran_sql);
    });
}

#[test]
fn should_exec_basicopt_with_where_with_none() {
    futures::executor::block_on(async move {
        let q = QueryBuilder::<Product2>::new().where_col(|c| c.name.equal(None));
        let client = welds_connections::noop::build(Syntax::Mssql);
        q.run(&client).await.unwrap();
        let ran_sql = client.last_sql().unwrap();
        let expected =
            "SELECT t1.\"dbname\" FROM da_schemaname.da_tablename t1 WHERE ( t1.dbname IS NULL )";
        assert_eq!(expected, &ran_sql);
    });
}

#[test]
fn should_exec_basicopt_with_where_with_some() {
    futures::executor::block_on(async move {
        let foo = Some("bar".to_string());
        let q = QueryBuilder::<Product2>::new().where_col(|c| c.name.equal(&foo));
        let client = welds_connections::noop::build(Syntax::Mssql);
        q.run(&client).await.unwrap();
        let ran_sql = client.last_sql().unwrap();
        let expected =
            "SELECT t1.\"dbname\" FROM da_schemaname.da_tablename t1 WHERE ( t1.dbname = @p1 )";
        assert_eq!(expected, &ran_sql);
    });
}
