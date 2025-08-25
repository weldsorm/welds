use welds::WeldsModel;

/*
 * NOTE: You shouldn't be writing Models by hand.
 * use the welds cli to generate models
 * The this model is here for the purpose of testing core itself
 * */

#[derive(Debug, WeldsModel, PartialEq)]
#[welds(table = "Products")]
#[welds(HasMany(orders, super::order::Order, "product_id"))]
pub struct Product {
    #[welds(rename = "product_id")]
    #[welds(primary_key)]
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price1: Option<f32>,
    pub price2: Option<f64>,
    pub active: Option<bool>,
}

#[derive(Debug, WeldsModel)]
#[welds(schema = "bad_schema", table = "Products")]
pub struct BadProductMissingTable {
    #[welds(primary_key)]
    #[welds(rename = "ID")]
    pub id: i32,
}

#[derive(Debug, WeldsModel)]
#[welds(table = "Products")]
pub struct BadProductColumns {
    #[welds(rename = "product_id")]
    #[welds(primary_key)]
    pub id: i64,
    pub name2: String,
    pub description: String,
    pub price1: Option<f64>,
}
