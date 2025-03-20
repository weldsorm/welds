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
