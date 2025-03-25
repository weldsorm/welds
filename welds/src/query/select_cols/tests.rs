use crate::Syntax;
use crate::WeldsModel;

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

#[test]
fn should_be_able_to_select_as() {
    futures::executor::block_on(async move {
        let q = Product::all().select_as(|x| x.id, "apples");
        let sql = q.to_sql(Syntax::Postgres);
        assert_eq!(sql, "SELECT t1.\"id\" AS \"apples\" FROM products t1");
    });
}

#[test]
fn should_be_able_to_select_both_sets_of_ids() {
    futures::executor::block_on(async move {
        let q = Product::all()
            .select_as(|x| x.id, "pid")
            .join(|x| x.orders, Order::all().select_as(|o| o.id, "oid"));
        let sql = q.to_sql(Syntax::Postgres);
        assert_eq!(sql, "SELECT t1.\"id\" AS \"pid\", t2.\"id\" AS \"oid\" FROM products t1 JOIN orders t2 ON t1.\"id\" = t2.\"product_id\"");
    });
}

#[test]
fn should_be_able_to_select_join_with_order_by() {
    futures::executor::block_on(async move {
        let q = Product::all()
            .select(|x| x.id)
            .join(|x| x.orders, Order::all().select_as(|o| o.price, "price"))
            .order_by_asc(|x| x.id);
        let sql = q.to_sql(Syntax::Postgres);
        assert_eq!(sql, "SELECT t1.\"id\", t2.\"price\" FROM products t1 JOIN orders t2 ON t1.\"id\" = t2.\"product_id\" ORDER BY t1.id ASC");
    });
}
