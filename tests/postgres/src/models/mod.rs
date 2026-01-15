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

#[derive(Debug, WeldsModel)]
#[welds(table = "StringThing")]
pub struct StringThing {
    #[welds(primary_key)]
    pub id: String,
    pub value: String,
}

#[derive(WeldsModel)]
#[welds(table = "uuid_id_from_db")]
pub struct UuidIdFromDb {
    #[welds(primary_key)]
    pub id: uuid::Uuid,
    pub name: String,
}

#[derive(WeldsModel)]
#[welds(table = "uuid_id_from_dev")]
pub struct UuidIdFromDev {
    #[welds(primary_key)]
    pub id: uuid::Uuid,
    pub name: String,
}

#[derive(WeldsModel)]
#[welds(table = "BadColumnNames")]
pub struct BadColumnNames {
    #[welds(primary_key)]
    #[welds(rename = " id")]
    pub id: i64,
    #[welds(rename = "camelCase")]
    pub camel_case: String,
    #[welds(rename = "col With     SPACES")]
    pub spaces: String,
    #[welds(rename = "col With -- DASH")]
    pub dash: String,
    #[welds(rename = "select")]
    pub keyword: String,
    #[welds(rename = "from 'quotes' ed")]
    pub quote: String,
}
