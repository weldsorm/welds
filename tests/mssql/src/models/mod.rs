pub mod order;
pub mod person;
pub mod persons_2;
pub mod product;

use welds::WeldsModel;

#[derive(Debug, WeldsModel)]
#[welds(schema = "welds", table = "Thing1")]
pub struct Thing1 {
    #[welds(primary_key)]
    pub id: i32,
    pub value: String,
}

#[derive(Debug, WeldsModel)]
#[welds(schema = "welds", table = "StringThing")]
pub struct StringThing {
    #[welds(primary_key)]
    pub id: String,
    pub value: String,
}
