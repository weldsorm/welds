pub mod enums;
pub mod order;
pub mod other;
pub mod product;
pub mod table_with_array;

use welds::WeldsModel;
#[derive(Debug, WeldsModel)]
#[welds(table = "Thing1")]
pub struct Thing1 {
    #[welds(primary_key)]
    pub id: i32,
    pub value: String,
}
#[derive(Debug, WeldsModel)]
#[welds(table = "Thing2")]
pub struct Thing2 {
    #[welds(primary_key)]
    pub id: i32,
    pub value: String,
}
