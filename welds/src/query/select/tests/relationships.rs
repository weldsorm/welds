use super::*;

// Test Object that can be used to write SQL
// Testing with null/some/and unwrapped values
use crate::WeldsModel;

// model with option
#[derive(Debug, WeldsModel)]
#[welds_path(crate)] // needed only within the welds crate.
#[welds(HasMany(orders, Order, "p_fk_id"))]
struct Product {
    #[welds(primary_key)]
    pub pid: i64,
}

#[derive(Debug, WeldsModel)]
#[welds_path(crate)] // needed only within the welds crate.
#[welds(BelongsTo(product, Product, "p_fk_id"))]
struct Order {
    #[welds(primary_key)]
    pub oid: i64,
    pub p_fk_id: i64,
}

// Tests

#[test]
fn should_select_through_an_entity() {
    futures::executor::block_on(async move {
        let q = QueryBuilder::<Product>::new()
            .where_col(|c| c.pid.equal(1))
            .map_query(|p| p.orders);
        let client = welds_connections::noop::build(Syntax::Mysql);
        q.run(&client).await.unwrap();
        let ran_sql = client.last_sql().unwrap();
        let expected = r#"SELECT t2.oid, t2.p_fk_id FROM order t2 WHERE ( EXISTS ( SELECT pid FROM product t1 WHERE t1.pid = ? AND t1.pid = t2.p_fk_id ) )"#;
        assert_eq!(expected, &ran_sql);
    });
}

#[test]
fn should_be_able_to_query_with_sub_queries() {
    futures::executor::block_on(async move {
        let products = QueryBuilder::<Product>::new().where_col(|c| c.pid.equal(1));
        let orders = QueryBuilder::<Order>::new().where_relation(|r| r.product, products);

        let client = welds_connections::noop::build(Syntax::Mysql);
        orders.run(&client).await.unwrap();
        let ran_sql = client.last_sql().unwrap();
        let expected = r#"SELECT t1.oid, t1.p_fk_id FROM order t1 WHERE ( EXISTS ( SELECT pid FROM product t2 WHERE t2.pid = ? AND t2.pid = t1.p_fk_id ) )"#;
        assert_eq!(expected, &ran_sql);
    });
}
