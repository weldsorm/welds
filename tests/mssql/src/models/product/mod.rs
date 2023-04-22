mod definition;
pub use definition::*;

use welds::WeldsModel;

#[derive(Debug, sqlx::FromRow, WeldsModel)]
#[welds(db(Mssql))]
#[welds(schema = "dbo", table = "Products")]
pub struct BadProductMissingTable {
    #[welds(primary_key)]
    #[sqlx(rename = "ID")]
    pub id: i32,
}

#[derive(Debug, sqlx :: FromRow, WeldsModel)]
#[welds(db(Mssql))]
#[welds(schema = "welds", table = "Products")]
pub struct BadProductColumns {
    #[welds(primary_key)]
    #[sqlx(rename = "id")]
    pub id: i32,
    pub active: i64,
    #[sqlx(rename = "Description")]
    pub description: Option<String>,
    //pub name: String,
    //#[sqlx(rename = "price1")]
    //pub price_1: Option<f32>,
    //#[sqlx(rename = "price2")]
    //pub price_2: Option<f32>,
}
