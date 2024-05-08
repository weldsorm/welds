pub mod order;
pub mod product;

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
pub struct Mysql {
    #[welds(primary_key)]
    pub id: i32,
    pub value: String,
}

#[derive(Debug, WeldsModel)]
#[welds(table = "StringThing")]
pub struct StringThing {
    #[welds(primary_key)]
    pub id: String,
    pub value: String,
}
