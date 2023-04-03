use welds_core::WeldsModel;

/*
 * NOTE: You shouldn't be writing Models by hand.
 * use the welds cli to generate models
 * The this model is here for the purpose of testing core itself
 * */

#[derive(Debug, sqlx::FromRow, WeldsModel)]
#[welds(db(Mssql))]
#[welds(schema = "welds", table = "Products")]
#[welds(HasMany(orders, super::order::Order, "product_id"))]
pub struct Product {
    #[welds(primary_key)]
    #[sqlx(rename = "ID")]
    pub id: i32,
    pub name: String,
    #[sqlx(rename = "Description")]
    pub description: Option<String>,
    pub price1: Option<f32>,
    pub price2: Option<f32>,
    pub active: Option<i32>,
}
