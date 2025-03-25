use welds::Syntax;
use welds::WeldsModel;

#[derive(Debug, WeldsModel)]
#[welds(db(Sqlite))]
#[welds(table = "orders")]
#[welds(BelongsTo(product, Product2, "product_id"))]
pub struct Order2 {
    #[welds(primary_key)]
    pub oid: i32,
    pub product_id: i32,
}
#[derive(Debug, WeldsModel)]
#[welds(db(Sqlite))]
#[welds(table = "Products")]
#[welds(HasMany(orders, Order2, "product_id"))]
pub struct Product2 {
    #[welds(primary_key)]
    pub pid: i32,
    pub name: String,
}

#[test]
fn be_able_to_delete_all() {
    async_std::task::block_on(async {
        let q = Order2::all().delete_sql(Syntax::Sqlite);
        assert_eq!(q, "DELETE FROM orders");
    })
}

#[test]
fn delete_from_with_where() {
    async_std::task::block_on(async {
        let q = Order2::all()
            .where_col(|x| x.oid.equal(1))
            .delete_sql(Syntax::Sqlite);
        assert_eq!(q, "DELETE FROM orders WHERE ( orders.oid = ? )");
    })
}

#[test]
fn should_be_able_to_delete_with_limit() {
    async_std::task::block_on(async {
        let q = Order2::all()
            .order_by_asc(|x| x.oid)
            .limit(3)
            .delete_sql(Syntax::Sqlite);
        assert_eq!(q, "DELETE FROM orders WHERE (  orders.oid IN (SELECT t1.\"oid\" FROM orders t1 ORDER BY t1.oid ASC LIMIT 3 OFFSET 0 )  )" );
    })
}

#[test]
fn should_be_able_to_delete_with_existsin() {
    async_std::task::block_on(async {
        let q = Product2::where_col(|p| p.pid.gt(1))
            .map_query(|p| p.orders)
            .delete_sql(Syntax::Sqlite);
        assert_eq!(q, "DELETE FROM orders WHERE ( EXISTS ( SELECT pid FROM Products t1 WHERE t1.pid > ? AND t1.pid = orders.product_id ) )" );
    })
}

#[test]
fn should_be_able_to_delete_with_wherein_with_limit() {
    async_std::task::block_on(async {
        let q = Product2::where_col(|p| p.pid.gt(1))
            .limit(1)
            .map_query(|p| p.orders)
            .delete_sql(Syntax::Sqlite);
        assert_eq!(q, "DELETE FROM orders WHERE (  orders.product_id IN (SELECT t1.pid FROM Products t1 WHERE t1.pid > ? ORDER BY 1 LIMIT 1 OFFSET 0 )  )" );
    })
}
