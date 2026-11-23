use crate::Syntax;
use crate::state::DbState;

// Testing that the tail end of the SQL is correct
// Limits / skips / orders
use crate::WeldsModel;

#[derive(Debug, Default, WeldsModel)]
#[welds(table = "nums")]
#[welds_path(crate)] // needed only within the welds crate.
struct Product {
    #[welds(primary_key)]
    pub id: i32,
    pub a: i32,
    pub b: i32,
}

// Tests

#[test]
fn should_be_able_to_insert_simple_object() {
    futures::executor::block_on(async move {
        let obj = Product::default();
        let mut obj = DbState::new_uncreated(obj);
        let client = welds_connections::noop::build(Syntax::Postgres);
        let _ = obj.save(&client).await;
        let ran_sql = client.last_sql().unwrap();

        let expected = "INSERT INTO nums (\"a\", \"b\") VALUES ($1, $2) RETURNING *";
        assert_eq!(expected, &ran_sql);
    });
}

#[derive(Debug, Default, WeldsModel)]
#[welds(table = "nums_zero_pk")]
#[welds_path(crate)]
struct ProductZeroPk {
    #[welds(primary_key(include_in_insert))]
    pub id: i32,
    pub a: i32,
}

#[test]
fn should_insert_zero_for_pk_when_include_in_insert_is_set() {
    futures::executor::block_on(async move {
        let mut obj = ProductZeroPk { id: 0, a: 42 };
        let mut obj = DbState::new_uncreated(obj);
        let client = welds_connections::noop::build(Syntax::Postgres);
        let _ = obj.save(&client).await;
        let ran_sql = client.last_sql().unwrap();

        let expected = "INSERT INTO nums_zero_pk (\"id\", \"a\") VALUES ($1, $2) RETURNING *";
        assert_eq!(expected, &ran_sql);
    });
}
