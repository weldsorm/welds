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

// The bad products are used to test
// validation structs are not wired up correctly

#[derive(Debug, WeldsModel)]
#[welds(table = "products")]
pub struct BadProduct1 {
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
#[welds(table = "Products")]
pub struct BadProduct2 {
    #[welds(rename = "product_id")]
    #[welds(primary_key)]
    pub id: f64,
    pub name2: String,
    pub description: String,
    //pub price1: Option<f32>,
    //pub price2: Option<f64>,
    //pub active: Option<bool>,
}

/// This is a version of a product that
/// can only edit the name.
/// and can read the description
#[derive(Debug, WeldsModel)]
#[welds(table = "Products")]
#[welds(HasMany(orders, super::order::Order, "product_id"))]
pub struct ProductNameOnly {
    #[welds(rename = "product_id")]
    #[welds(primary_key)]
    pub id: i32,
    pub name: String,
    #[welds(readonly)]
    pub description: Option<String>,
}
