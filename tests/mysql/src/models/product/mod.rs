use welds::WeldsModel;

/*
 * NOTE: You shouldn't be writing Models by hand.
 * use the welds cli to generate models
 * The this model is here for the purpose of testing core itself
 * */

#[derive(Debug, sqlx::FromRow, WeldsModel)]
#[welds(db(Mysql))]
#[welds(table = "Products")]
#[welds(HasMany(orders, super::order::Order, "product_id"))]
pub struct Product {
    #[sqlx(rename = "product_id")]
    #[welds(primary_key)]
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price1: Option<f32>,
    pub price2: Option<f64>,
    pub active: Option<bool>,
}

#[derive(Debug, sqlx::FromRow, WeldsModel)]
#[welds(db(Mysql))]
#[welds(schema = "bad_schema", table = "Products")]
pub struct BadProductMissingTable {
    #[welds(primary_key)]
    #[sqlx(rename = "ID")]
    pub id: i32,
}

#[derive(Debug, sqlx::FromRow, WeldsModel)]
#[welds(db(Mysql))]
#[welds(table = "Products")]
pub struct BadProductColumns {
    #[sqlx(rename = "product_id")]
    #[welds(primary_key)]
    pub id: i64,
    pub name2: String,
    pub description: String,
    pub price1: Option<f64>,
    //pub price2: Option<f64>,
    //pub active: Option<bool>,
}
