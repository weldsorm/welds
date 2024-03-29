use welds::WeldsModel;

/*
 * NOTE: You shouldn't be writing Models by hand.
 * use the welds cli to generate models
 * The this model is here for the purpose of testing core itself
 * */

#[derive(Debug, WeldsModel)]
#[welds(table = "orders")]
#[welds(BelongsTo(product, super::product::Product, "product_id"))]
pub struct Order {
    #[welds(primary_key)]
    pub id: i32,
    pub product_id: i32,
    pub code: Option<String>,
}

#[derive(Debug, WeldsModel)]
#[welds(table = "orders")]
pub struct SmallOrder {
    #[welds(primary_key)]
    pub id: i32,
    pub product_id: i32,
    pub code: Option<String>,
}
