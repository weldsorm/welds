mod definition;
pub use definition::*;
use welds::WeldsModel;

#[derive(Debug, WeldsModel)]
#[welds(table = "Products")]
pub struct ProductManualPk {
    #[welds(rename = "product_id")]
    #[welds(primary_key(include_in_insert))]
    pub id: i32,
    pub name: String,
    pub description: String,
}
