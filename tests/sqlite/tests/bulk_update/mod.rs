use welds::Syntax;
use welds::WeldsModel;

#[derive(Debug, WeldsModel)]
#[welds(table = "orders")]
#[welds(BelongsTo(product, Product2, "product_id"))]
pub struct Order2 {
    #[welds(primary_key)]
    pub oid: i32,
    pub product_id: i32,
}
#[derive(Debug, WeldsModel)]
#[welds(table = "Products")]
#[welds(HasMany(orders, Order2, "product_id"))]
pub struct Product2 {
    #[welds(primary_key)]
    pub pid: i32,
    pub name: String,
}

#[test]
fn should_be_able_to_update_table() {
    async_std::task::block_on(async {
        let q = Product2::all()
            .set(|x| x.name, "Test")
            .to_sql(Syntax::Sqlite);
        assert_eq!(q, "UPDATE Products SET \"name\"=?");
    })
}

#[test]
fn should_be_able_to_update_table_with_where() {
    async_std::task::block_on(async {
        let q = Product2::all()
            .where_col(|x| x.pid.equal(1))
            .set(|x| x.name, "Test")
            .to_sql(Syntax::Sqlite);
        assert_eq!(
            q,
            "UPDATE Products SET \"name\"=? WHERE ( Products.\"pid\" = ? )"
        );
    })
}

#[test]
fn should_be_able_to_update_table_with_where_exists_from_map_query() {
    async_std::task::block_on(async {
        let q = Order2::where_col(|o| o.oid.equal(1))
            .map_query(|o| o.product)
            .set(|x| x.name, "Test")
            .to_sql(Syntax::Sqlite);
        assert_eq!(
            q,
            "UPDATE Products SET \"name\"=? WHERE ( EXISTS ( SELECT \"product_id\" FROM orders t1 WHERE t1.\"oid\" = ? AND t1.\"product_id\" = Products.\"pid\" ) )"
        );
    })
}

#[test]
fn should_be_able_to_update_table_with_where_in_from_map_query_with_limit() {
    async_std::task::block_on(async {
        let q = Order2::where_col(|o| o.oid.equal(1))
            .limit(1)
            .map_query(|o| o.product)
            .set(|x| x.name, "Test")
            .to_sql(Syntax::Sqlite);
        assert_eq!(
            q,
            "UPDATE Products SET \"name\"=? WHERE (  Products.\"pid\" IN (SELECT t1.\"product_id\" FROM orders t1 WHERE t1.\"oid\" = ? ORDER BY 1 LIMIT 1 OFFSET 0 )  )"
        );
    })
}

#[test]
fn should_be_able_to_update_with_just_limit() {
    async_std::task::block_on(async {
        let q = Product2::all()
            .limit(1)
            .set(|x| x.name, "Test")
            .to_sql(Syntax::Sqlite);
        assert_eq!(
            q,
            "UPDATE Products SET \"name\"=? WHERE (  Products.\"pid\" IN (SELECT t1.\"pid\" FROM Products t1 ORDER BY 1 LIMIT 1 OFFSET 0 )  )"
        );
    })
}
