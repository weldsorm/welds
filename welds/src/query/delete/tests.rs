//use super::*;
use crate::errors::Result;
use crate::model_traits::Column;
use crate::model_traits::HasSchema;
use crate::model_traits::TableColumns;
use crate::model_traits::TableInfo;
use crate::model_traits::UniqueIdentifier;
use crate::model_traits::WriteToArgs;
use crate::query::clause::Numeric;
use crate::query::clause::NumericOpt;
use crate::query::clause::ParamArgs;
use crate::state::DbState;
use crate::Syntax;
use welds_connections::Row;

// Testing that the tail end of the SQL is correct
// Limits / skips / orders

#[derive(Debug, Default)]
struct Product {
    pub id: i32,
    pub a: i32,
    pub b: i32,
}

impl TryFrom<Row> for Product {
    type Error = crate::WeldsError;
    fn try_from(value: Row) -> std::result::Result<Self, Self::Error> {
        Ok(Product { id: 1, a: 0, b: 0 })
    }
}

pub struct ProductSchema {
    id: Numeric<i32>,
    a: Numeric<i32>,
    b: NumericOpt<i32>,
}

impl Default for ProductSchema {
    fn default() -> Self {
        Self {
            id: Numeric::new("id", "id"),
            a: Numeric::new("a", "a"),
            b: NumericOpt::new("b", "b"),
        }
    }
}

impl WriteToArgs for Product {
    fn bind<'s, 'c, 'a>(&'s self, column: &'c str, args: &'s mut ParamArgs<'a>) -> Result<()> {
        Ok(())
    }
}

impl TableInfo for ProductSchema {
    fn identifier() -> &'static [&'static str] {
        &["nums"]
    }
}

impl TableColumns for ProductSchema {
    fn columns() -> Vec<Column> {
        vec![
            Column::mock("id", false),
            Column::mock("a", false),
            Column::mock("b", true),
        ]
    }
    fn primary_keys() -> Vec<Column> {
        vec![Column::mock("id", false)]
    }
}

impl HasSchema for Product {
    type Schema = ProductSchema;
}

impl UniqueIdentifier for ProductSchema {
    fn id_column() -> Column {
        Column::mock("id", false)
    }
}

// Tests

#[test]
fn should_be_able_to_delete_simple_object() {
    futures::executor::block_on(async move {
        let obj = Product::default();
        let mut obj = DbState::db_loaded(obj);
        let client = welds_connections::noop::build(Syntax::Mysql);
        let _ = obj.delete(&client).await;
        let ran_sql = client.last_sql().unwrap();
        let expected = "DELETE FROM nums where id=?";
        assert_eq!(expected, &ran_sql);
    });
}

#[test]
fn should_be_able_to_write_delete_query() {
    futures::executor::block_on(async move {
        use crate::query::builder::QueryBuilder;
        let q = QueryBuilder::<Product>::new().where_col(|c| c.a.gt(1));
        let client = welds_connections::noop::build(Syntax::Mysql);
        let _ = q.delete(&client).await;
        let ran_sql = client.last_sql().unwrap();
        let expected = "DELETE FROM nums WHERE ( nums.a > ? )";
        assert_eq!(expected, &ran_sql);
    });
}

#[test]
fn should_be_able_to_write_delete_query_with_limit() {
    futures::executor::block_on(async move {
        use crate::query::builder::QueryBuilder;
        let q = QueryBuilder::<Product>::new()
            .where_col(|c| c.a.gt(1))
            .limit(10);
        let client = welds_connections::noop::build(Syntax::Mysql);
        let _ = q.delete(&client).await;
        let ran_sql = client.last_sql().unwrap();
        let expected = "DELETE FROM nums WHERE (  nums.id IN (SELECT t1.id FROM nums t1 WHERE ( t1.a > ? ) ORDER BY 1 LIMIT 0, 10)  )";
        assert_eq!(expected, &ran_sql);

        let args_count = client.args_count().unwrap();
        assert_eq!(args_count, 1);
    });
}
