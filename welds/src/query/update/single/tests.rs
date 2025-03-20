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
fn no_changes_should_do_nothing() {
    futures::executor::block_on(async move {
        let obj = Product::default();
        let mut obj = DbState::db_loaded(obj);
        let client = welds_connections::noop::build(Syntax::Postgres);
        let _ = obj.save(&client).await;
        let ran_sql = client.last_sql();
        assert!(ran_sql.is_none());
    });
}

#[test]
fn changing_a_value_should_update() {
    futures::executor::block_on(async move {
        let obj = Product::default();
        let mut obj = DbState::db_loaded(obj);
        obj.a = 42;
        let client = welds_connections::noop::build(Syntax::Postgres);
        let _ = obj.save(&client).await;
        let ran_sql = client
            .last_sql()
            .expect("Expected to get SQL back from client");
        let expected = "UPDATE nums SET \"a\"=$1, \"b\"=$2 where \"id\"=$3";
        assert_eq!(expected, &ran_sql);
    });
}
