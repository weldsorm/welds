use crate::WeldsModel;
use welds_connections::Syntax;

#[derive(Debug, Default, WeldsModel)]
#[welds(table = "products")]
#[welds_path(crate)] // needed only within the welds crate.
#[welds(HasMany(orders, OrderC, "product_id"))]
struct ProductC {
    #[welds(primary_key)]
    pub pid: i32,
    pub name: String,
}

#[derive(Debug, Default, WeldsModel)]
#[welds(table = "orders")]
#[welds_path(crate)] // needed only within the welds crate.
#[welds(BelongsTo(product, ProductC, "product_id"))]
struct OrderC {
    #[welds(primary_key)]
    pub id: i32,
    #[welds(rename = "product_id")]
    pub prod_id: i32,
    pub price: i32,
}

#[test]
fn should_be_able_to_map_query_from_belongs_to() {
    futures::executor::block_on(async move {
        let q = ProductC::all().map_query(|p| p.orders);
        let sql = q.to_sql(Syntax::Mssql);
        let valid = r#"SELECT t2."id", t2."product_id", t2."price" FROM orders t2 WHERE ( EXISTS ( SELECT pid FROM products t1 WHERE t1.pid = t2.product_id ) )"#;
        assert_eq!(sql, valid);
    });
}

#[test]
fn should_be_able_to_map_query_from_has_many() {
    futures::executor::block_on(async move {
        let q = OrderC::all().map_query(|p| p.product);
        let sql = q.to_sql(Syntax::Mssql);
        let valid = r#"SELECT t2."pid", t2."name" FROM products t2 WHERE ( EXISTS ( SELECT product_id FROM orders t1 WHERE t1.product_id = t2.pid ) )"#;
        assert_eq!(sql, valid);
    });
}
