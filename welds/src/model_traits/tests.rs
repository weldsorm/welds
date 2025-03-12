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
fn should_be_able_to_read_the_pk() {
    futures::executor::block_on(async move {
        let p = Product {
            id: 33,
            name: Default::default(),
        };
        let value = super::PrimaryKeyValue::value(&p);
        assert_eq!(33, value);
    });
}
