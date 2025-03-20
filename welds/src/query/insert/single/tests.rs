use crate::state::DbState;
use crate::Syntax;

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
