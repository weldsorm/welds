use super::DataSet;
use crate::state::DbState;
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

//fn mock_dataset() -> DataSet<Product> {
//    todo!()
//}
//
//#[test]
//fn should_be_able_to_iter_over_dataset() {
//    let set = mock_dataset();
//    for p in set.iter() {
//        // make sure we can access the Products content
//        println!("p: {}", p.id);
//        let orders: Option<&[DbState<Order>]> = p.get(|p| p.orders);
//    }
//}
