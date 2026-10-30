use super::*;
use crate::Syntax;
use crate::WeldsModel;
use crate::model_traits::{HasSchema, TableInfo};
use crate::query::builder::QueryBuilder;
use crate::query::clause::exists::ExistIn;
use crate::query::select_cols::Join;

#[derive(Debug, Default, WeldsModel)]
#[welds(table = "products")]
#[welds_path(crate)] // needed only within the welds crate.
#[welds(HasMany(orders, Order, "product_id"))]
struct Product {
    #[welds(primary_key)]
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Default, WeldsModel)]
#[welds(table = "orders")]
#[welds_path(crate)] // needed only within the welds crate.
#[welds(BelongsTo(product, Product, "product_id"))]
struct Order {
    #[welds(primary_key)]
    pub id: i32,
    pub product_id: i32,
    pub price: i32,
}

/// Wire up a relationship as a sub-query the same way `where_relation` does.
fn where_ship<T, R>(
    mut outer: QueryBuilder<T>,
    ship: ManualRelationship<T, R>,
    filter: QueryBuilder<R>,
) -> QueryBuilder<T>
where
    T: Send,
    R: Send + HasSchema,
    <R as HasSchema>::Schema: TableInfo,
{
    let writer = RelationshipWriter::new(&ship, Side::MineIsOuter);
    let inner_tn = <R as HasSchema>::Schema::identifier();
    let mut exist_in = ExistIn::new(&filter, inner_tn, writer);
    exist_in.set_aliases(&outer.alias_asigner);
    outer.exist_ins.push(exist_in);
    outer
}

/// Wire up a relationship as a sub-query the same way `map_query` and `include` do.
/// The relationship is defined on T, but T is the inner query
fn map_ship<T, R>(inner: &QueryBuilder<T>, ship: ManualRelationship<T, R>) -> QueryBuilder<R>
where
    T: Send + HasSchema,
    <T as HasSchema>::Schema: TableInfo,
    R: Send + HasSchema,
{
    let mut qb: QueryBuilder<R> = QueryBuilder::new();
    qb.set_aliases(&inner.alias_asigner);
    let writer = RelationshipWriter::new(&ship, Side::MineIsInner);
    let inner_tn = <T as HasSchema>::Schema::identifier();
    qb.exist_ins.push(ExistIn::new(inner, inner_tn, writer));
    qb
}

#[test]
fn join_manual_should_match_join_with_for_a_simple_relationship() {
    let ship = ManualRelationship::<Product, Order>::using("id", "product_id");
    let manual = Product::all().select(|p| p.id).join_manual(
        Join::Inner,
        ship,
        Order::all().select(|o| o.price),
    );
    let derived = Product::all()
        .select(|p| p.id)
        .join(|p| p.orders, Order::all().select(|o| o.price));
    assert_eq!(
        manual.to_sql(Syntax::Postgres),
        derived.to_sql(Syntax::Postgres)
    );
}

#[test]
fn join_manual_should_write_all_clauses_in_the_on() {
    let ship = ManualRelationship::<Product, Order>::using("id", "product_id")
        .and_using("name", "price")
        .and_their_eq("price", 10)
        .and_my_eq("name", "horse".to_string());
    let q = Product::all().select(|p| p.id).join_manual(
        Join::Left,
        ship,
        Order::all().select(|o| o.price),
    );
    assert_eq!(
        q.to_sql(Syntax::Postgres),
        "SELECT t1.\"id\", t2.\"price\" FROM products t1 \
        LEFT JOIN orders t2 ON t1.\"id\" = t2.\"product_id\" AND t1.\"name\" = t2.\"price\" \
        AND t2.\"price\" = $1 AND t1.\"name\" = $2"
    );
}

#[test]
fn join_params_should_be_numbered_before_where_params() {
    futures::executor::block_on(async move {
        let ship = ManualRelationship::<Product, Order>::using("id", "product_id")
            .and_their_eq("price", 10);
        let q = Product::all()
            .where_col(|p| p.id.gt(5))
            .select(|p| p.id)
            .join_manual(
                Join::Inner,
                ship,
                Order::where_col(|o| o.price.lt(100)).select(|o| o.price),
            );
        let expected = "SELECT t1.\"id\", t2.\"price\" FROM products t1 \
            JOIN orders t2 ON t1.\"id\" = t2.\"product_id\" AND t2.\"price\" = $1 \
            WHERE ( t1.\"id\" > $2 AND t2.\"price\" < $3 )";
        assert_eq!(q.to_sql(Syntax::Postgres), expected);

        // make sure every placeholder has an arg
        let client = welds_connections::noop::build(Syntax::Postgres);
        q.run(&client).await.unwrap();
        assert_eq!(client.last_sql().unwrap(), expected);
        assert_eq!(client.args_count(), Some(3));
    });
}

#[test]
fn nested_joins_should_number_params_in_the_order_written() {
    let order_to_product =
        ManualRelationship::<Order, Product>::using("product_id", "id").and_their_eq("name", 1);
    let product_to_order =
        ManualRelationship::<Product, Order>::using("id", "product_id").and_their_eq("price", 2);
    let inner = Order::all().select(|o| o.id).join_manual(
        Join::Inner,
        order_to_product,
        Product::all().select(|p| p.name),
    );
    let q = Product::all()
        .select(|p| p.id)
        .join_manual(Join::Inner, product_to_order, inner);
    assert_eq!(
        q.to_sql(Syntax::Postgres),
        "SELECT t1.\"id\", t2.\"id\", t3.\"name\" FROM products t1 \
        JOIN orders t2 ON t1.\"id\" = t2.\"product_id\" AND t2.\"price\" = $1 \
        JOIN products t3 ON t2.\"product_id\" = t3.\"id\" AND t3.\"name\" = $2"
    );
}

#[test]
fn sub_query_should_write_all_clauses() {
    futures::executor::block_on(async move {
        let ship = ManualRelationship::<Order, Product>::using("product_id", "id")
            .and_their_eq("name", "horse".to_string())
            .and_my_eq("price", 7);
        let q = where_ship(
            Order::where_col(|o| o.price.gt(1)),
            ship,
            Product::where_col(|p| p.id.lt(9)),
        );
        let expected = "SELECT t1.\"id\", t1.\"product_id\", t1.\"price\" FROM orders t1 WHERE ( \
            t1.\"price\" > $1 AND EXISTS ( SELECT \"id\" FROM products t2 WHERE \
            t2.\"id\" < $2 AND t2.\"id\" = t1.\"product_id\" AND t2.\"name\" = $3 AND t1.\"price\" = $4 ) )";
        let client = welds_connections::noop::build(Syntax::Postgres);
        q.run(&client).await.unwrap();
        assert_eq!(client.last_sql().unwrap(), expected);
        assert_eq!(client.args_count(), Some(4));
    });
}

#[test]
fn sub_query_should_flip_sides_when_mapping_through_a_relationship() {
    // relationship defined on Order, but Order is the inner query
    let ship = ManualRelationship::<Order, Product>::using("product_id", "id")
        .and_my_eq("price", 7)
        .and_their_eq("name", "horse".to_string());
    let orders = Order::where_col(|o| o.id.equal(1));
    let products = map_ship(&orders, ship);
    assert_eq!(
        products.to_sql(Syntax::Postgres),
        "SELECT t2.\"id\", t2.\"name\" FROM products t2 WHERE ( EXISTS ( SELECT \"product_id\" FROM orders t1 WHERE \
        t1.\"id\" = $1 AND t1.\"product_id\" = t2.\"id\" AND t1.\"price\" = $2 AND t2.\"name\" = $3 ) )"
    );
}

#[test]
fn limited_sub_query_should_use_in_with_the_first_key_pair() {
    let ship = ManualRelationship::<Order, Product>::using("product_id", "id")
        .and_their_eq("name", "horse".to_string());
    let q = where_ship(Order::all(), ship, Product::all().limit(2));
    let sql = q.to_sql(Syntax::Postgres);
    assert!(
        sql.contains(
            "t1.\"product_id\" IN (SELECT t2.\"id\" FROM products t2 WHERE t2.\"name\" = $1"
        ),
        "{}",
        sql
    );
    assert!(!sql.contains("t2.\"id\" = t1.\"product_id\""), "{}", sql);
}

#[test]
fn sub_query_without_a_key_pair_should_use_exists() {
    let ship =
        ManualRelationship::<Order, Product>::empty().and_their_eq("name", "horse".to_string());
    let q = where_ship(Order::all(), ship, Product::all().limit(2));
    let sql = q.to_sql(Syntax::Postgres);
    assert!(
        sql.contains("EXISTS ( SELECT 1 FROM products t2 WHERE t2.\"name\" = $1 "),
        "{}",
        sql
    );
    assert!(sql.contains("LIMIT 2"), "{}", sql);
    assert!(!sql.contains(" IN "), "{}", sql);
}

#[test]
fn manual_relationships_should_compare_by_clauses() {
    let a = ManualRelationship::<Order, Product>::using("product_id", "id");
    let b = ManualRelationship::<Order, Product>::using("product_id", "id");
    assert!(a == b);
    let c = ManualRelationship::<Order, Product>::using("product_id", "id").and_using("a", "b");
    assert!(a != c);

    // values can't be compared directly, clones share a value and are equal
    let d = a.clone().and_their_eq("name", 1);
    assert!(d == d.clone());
    assert!(d != a.clone().and_their_eq("name", 1));
}

#[test]
fn builtin_relationships_should_describe_a_single_key_pair() {
    let ship = <Order as HasRelations>::Relation::default().product;
    let clauses = Relationship::<Order, Product>::clauses(&ship);
    assert_eq!(clauses.len(), 1);
    let kp = clauses[0].as_key_pair().unwrap();
    assert_eq!(kp, &KeyPair::new("product_id", "id"));

    let ship = <Product as HasRelations>::Relation::default().orders;
    let clauses = Relationship::<Product, Order>::clauses(&ship);
    let kp = clauses[0].as_key_pair().unwrap();
    assert_eq!(kp, &KeyPair::new("id", "product_id"));
}
