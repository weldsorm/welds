//use super::*;
use crate::errors::Result;
use crate::model_traits::Column;
use crate::model_traits::HasSchema;
use crate::model_traits::TableColumns;
use crate::model_traits::TableInfo;
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
