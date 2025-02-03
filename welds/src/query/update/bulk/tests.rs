use crate::query::builder::QueryBuilder;
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
fn should_be_able_to_write_simple_set_value() {
    futures::executor::block_on(async move {
        // setup the query
        let q = QueryBuilder::<Product>::new();
        let bulk = q.set(|p| p.a, 1);

        //setup the fake client and run it.
        let client = welds_connections::noop::build(Syntax::Postgres);
        let _ = bulk.run(&client).await;

        let ran_sql = client
            .last_sql()
            .expect("Expected to get SQL back from client");

        let expected = "UPDATE nums SET \"a\"=$1";
        assert_eq!(expected, &ran_sql);

        assert_eq!(client.args_count().unwrap(), 1);
    });
}

#[test]
fn should_be_able_to_write_complex_set_values() {
    futures::executor::block_on(async move {
        // setup the query
        let q = QueryBuilder::<Product>::new().where_col(|c| c.id.gt(10));
        let bulk = q.set(|p| p.a, 1).set(|p| p.b, 2);

        //setup the fake client and run it.
        let client = welds_connections::noop::build(Syntax::Postgres);
        let _ = bulk.run(&client).await;

        let ran_sql = client
            .last_sql()
            .expect("Expected to get SQL back from client");

        let expected = "UPDATE nums SET \"a\"=$1, \"b\"=$2 WHERE ( nums.id > $3 )";
        assert_eq!(expected, &ran_sql);

        assert_eq!(client.args_count().unwrap(), 3);
    });
}

#[test]
fn should_be_able_to_write_complex_set_col_values() {
    futures::executor::block_on(async move {
        let q = QueryBuilder::<Product>::new().where_col(|c| c.id.gt(10));

        let bulk = q.set_col(|x| x.a.equal(2)).set_col(|p| p.b.equal(2));

        //setup the fake client and run it.
        let client = welds_connections::noop::build(Syntax::Postgres);
        let _ = bulk.run(&client).await;

        let ran_sql = client
            .last_sql()
            .expect("Expected to get SQL back from client");

        //let expected = "UPDATE nums SET nums.a = $1, nums.b = $2 WHERE ( nums.id > $3 )";
        let expected = "UPDATE nums SET \"a\"=$1, \"b\"=$2 WHERE ( nums.id > $3 )";
        assert_eq!(expected, &ran_sql);
    });
}
