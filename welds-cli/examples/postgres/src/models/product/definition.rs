
/******************************************************************************
 * This file was auto-generated by welds-cli. 
 * changes to this file will be overridden when the welds-cli generate command runs again.
******************************************************************************/


use welds::WeldsModel;
#[derive(Debug, sqlx :: FromRow, WeldsModel)]
#[welds(db(Postgres))]
#[welds(schema = "public", table = "products")]
pub struct Product {
    #[welds(primary_key)]
    pub product_id: i32,
    pub active: bool,
    pub barcode: Vec<u8>,
    pub description: String,
    pub name: String,
    pub price_1: f32,
    pub price_2: f64,
    pub price_3: sqlx::postgres::types::PgMoney,
}