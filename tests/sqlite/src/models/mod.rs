pub mod order;
pub mod product;
use welds::WeldsModel;

#[derive(Debug, WeldsModel)]
#[welds(db(Sqlite))]
#[welds(table = "Thing1")]
pub struct Thing1 {
    #[welds(primary_key)]
    pub id: i32,
    pub value: String,
}

#[derive(Debug, WeldsModel)]
#[welds(db(Sqlite))]
#[welds(table = "Thing2")]
pub struct Thing2 {
    #[welds(primary_key)]
    pub id: i32,
    pub value: String,
}

#[derive(Debug, WeldsModel)]
#[welds(db(Sqlite))]
#[welds(table = "Thing3")]
pub struct Thing3 {
    #[welds(primary_key)]
    // we should still be able to use the table with a small type
    pub id: i16,
    pub value: String,
}
