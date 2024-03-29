mod definition;
pub use definition::*;

use welds::WeldsModel;

#[derive(Debug, WeldsModel)]
#[welds(schema = "dbo", table = "Products")]
pub struct BadProductMissingTable {
    #[welds(primary_key)]
    #[welds(rename = "ID")]
    pub id: i32,
}

#[derive(Debug, WeldsModel)]
#[welds(schema = "welds", table = "Products")]
pub struct BadProductColumns {
    #[welds(primary_key)]
    #[welds(rename = "id")]
    pub id: i32,
    pub active: i64,
    #[welds(rename = "Description")]
    pub description: Option<String>,
}
