pub mod order;
pub mod product;

use welds::WeldsModel;
#[derive(Debug, sqlx::FromRow, WeldsModel)]
#[welds(db(Mysql))]
#[welds(table = "Thing1")]
pub struct Thing1 {
    #[welds(primary_key)]
    pub id: i32,
    pub value: String,
}
#[derive(Debug, sqlx::FromRow, WeldsModel)]
#[welds(db(Postgres))]
#[welds(table = "Thing2")]
pub struct Mysql {
    #[welds(primary_key)]
    pub id: i32,
    pub value: String,
}
