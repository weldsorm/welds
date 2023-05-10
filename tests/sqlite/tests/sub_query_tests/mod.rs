type Db = sqlx::Sqlite;
use welds::WeldsModel;

#[derive(Debug, sqlx::FromRow, WeldsModel)]
#[welds(db(Sqlite))]
#[welds(table = "orders")]
#[welds(BelongsTo(product, Product2, "product_id"))]
pub struct Order2 {
    #[welds(primary_key)]
    pub oid: i32,
    pub product_id: i32,
}
#[derive(Debug, sqlx::FromRow, WeldsModel)]
#[welds(db(Sqlite))]
#[welds(table = "Products")]
#[welds(HasMany(orders, Order2, "product_id"))]
pub struct Product2 {
    #[welds(primary_key)]
    pub pid: i32,
    pub name: String,
}

#[test]
fn exist_in_mapping_query_source_belongs() {
    async_std::task::block_on(async {
        let q1 = Order2::all::<Db>().map_query(|o| o.product).to_sql();
        assert_eq!(q1, "SELECT t1.\"pid\", t1.\"name\" FROM Products t1 WHERE ( EXISTS ( SELECT product_id FROM orders t2 WHERE t2.product_id = t1.pid ) )");
        let q2 = Order2::all::<Db>()
            .limit(1)
            .map_query(|o| o.product)
            .to_sql();
        assert_eq!(q2, "SELECT t1.\"pid\", t1.\"name\" FROM Products t1 WHERE (  t1.pid IN (SELECT t2.product_id FROM orders t2  ORDER BY 1 LIMIT 1 OFFSET 0 )  )");
    })
}

#[test]
fn exist_in_mapping_query_source_many() {
    async_std::task::block_on(async {
        let q1 = Product2::all::<Db>().map_query(|o| o.orders).to_sql();
        assert_eq!(q1, "SELECT t1.\"oid\", t1.\"product_id\" FROM orders t1 WHERE ( EXISTS ( SELECT pid FROM Products t2 WHERE t2.pid = t1.product_id ) )");
        let q2 = Product2::all::<Db>()
            .limit(1)
            .map_query(|o| o.orders)
            .to_sql();
        assert_eq!(q2, "SELECT t1.\"oid\", t1.\"product_id\" FROM orders t1 WHERE (  t1.product_id IN (SELECT t2.pid FROM Products t2  ORDER BY 1 LIMIT 1 OFFSET 0 )  )");
    })
}

#[test]
fn exist_in_sub_query_source_belongs() {
    async_std::task::block_on(async {
        let sub = Order2::all::<Db>();
        let q1 = Product2::all::<Db>()
            .where_relation(|o| o.orders, sub)
            .to_sql();
        assert_eq!(q1, "SELECT t1.\"pid\", t1.\"name\" FROM Products t1 WHERE ( EXISTS ( SELECT product_id FROM orders t2 WHERE t2.product_id = t1.pid ) )");

        let sub2 = Order2::all::<Db>().limit(1);
        let q2 = Product2::all::<Db>()
            .where_relation(|o| o.orders, sub2)
            .to_sql();
        assert_eq!(q2, "SELECT t1.\"pid\", t1.\"name\" FROM Products t1 WHERE (  t1.pid IN (SELECT t2.product_id FROM orders t2  ORDER BY 1 LIMIT 1 OFFSET 0 )  )");
    })
}

#[test]
fn exist_in_sub_query_source_many() {
    async_std::task::block_on(async {
        let sub1 = Product2::all::<Db>();
        let q1 = Order2::all::<Db>()
            .where_relation(|o| o.product, sub1)
            .to_sql();
        assert_eq!(q1, "SELECT t1.\"oid\", t1.\"product_id\" FROM orders t1 WHERE ( EXISTS ( SELECT pid FROM Products t2 WHERE t2.pid = t1.product_id ) )");

        let sub2 = Product2::all::<Db>().limit(1);
        let q2 = Order2::all::<Db>()
            .where_relation(|o| o.product, sub2)
            .to_sql();
        assert_eq!(q2, "SELECT t1.\"oid\", t1.\"product_id\" FROM orders t1 WHERE (  t1.product_id IN (SELECT t2.pid FROM Products t2  ORDER BY 1 LIMIT 1 OFFSET 0 )  )");
    })
}
