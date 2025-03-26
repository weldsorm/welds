use super::*;
use crate::model_traits::Column;
use crate::model_traits::HasSchema;
use crate::model_traits::TableColumns;
use crate::model_traits::TableInfo;
use crate::query::clause::Numeric;
use crate::query::clause::NumericOpt;

// Testing that the tail end of the SQL is correct
// Limits / skips / orders

struct Product {
    pub a: i32,
    pub b: i32,
}

impl TryFrom<Row> for Product {
    type Error = crate::WeldsError;
    fn try_from(_value: Row) -> std::result::Result<Self, Self::Error> {
        Ok(Product { a: 0, b: 0 })
    }
}

pub struct ProductSchema {
    a: Numeric<i32>,
    b: NumericOpt<i32>,
}

impl Default for ProductSchema {
    fn default() -> Self {
        Self {
            a: Numeric::new("a", "a"),
            b: NumericOpt::new("b", "b"),
        }
    }
}

impl TableInfo for ProductSchema {
    fn identifier() -> &'static [&'static str] {
        &["nums"]
    }
}

impl TableColumns for ProductSchema {
    type ColumnStruct = Self;
    fn columns() -> Vec<Column> {
        vec![
            Column::new("a", "i32", false),
            Column::new("b", "i32", true),
        ]
    }
    fn primary_keys() -> Vec<Column> {
        vec![]
    }
}

impl HasSchema for Product {
    type Schema = ProductSchema;
}

// Tests

#[test]
fn should_order_by_asc() {
    let q = QueryBuilder::<Product>::new().order_by_asc(|x| x.a);
    let ran_sql = futures::executor::block_on(async move {
        let client = welds_connections::noop::build(Syntax::Mssql);
        q.run(&client).await.unwrap();
        client.last_sql()
    })
    .unwrap();
    assert_eq!(
        "SELECT t1.\"a\", t1.\"b\" FROM nums t1 ORDER BY t1.a ASC",
        &ran_sql
    );
}

#[test]
fn should_order_by_two_columns() {
    let q = QueryBuilder::<Product>::new()
        .order_by_asc(|x| x.a)
        .order_by_desc(|x| x.b);
    let ran_sql = futures::executor::block_on(async move {
        let client = welds_connections::noop::build(Syntax::Sqlite);
        q.run(&client).await.unwrap();
        client.last_sql()
    })
    .unwrap();
    assert_eq!(
        "SELECT t1.\"a\", t1.\"b\" FROM nums t1 ORDER BY t1.a ASC, t1.b DESC",
        &ran_sql
    );
}

#[test]
fn should_be_able_to_limit() {
    let q = QueryBuilder::<Product>::new().limit(10);
    let ran_sql = futures::executor::block_on(async move {
        let client = welds_connections::noop::build(Syntax::Sqlite);
        q.run(&client).await.unwrap();
        client.last_sql()
    })
    .unwrap();
    assert_eq!(
        "SELECT t1.\"a\", t1.\"b\" FROM nums t1 ORDER BY 1 LIMIT 10 OFFSET 0 ",
        &ran_sql
    );
}

#[test]
fn should_be_able_to_offset() {
    let q = QueryBuilder::<Product>::new().limit(4).offset(15);
    let ran_sql = futures::executor::block_on(async move {
        let client = welds_connections::noop::build(Syntax::Postgres);
        q.run(&client).await.unwrap();
        client.last_sql()
    })
    .unwrap();
    assert_eq!(
        "SELECT t1.\"a\", t1.\"b\" FROM nums t1 ORDER BY 1 OFFSET 15 LIMIT 4",
        &ran_sql
    );
}
