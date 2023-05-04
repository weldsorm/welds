pub mod order;
pub mod person;
pub mod persons_2;
pub mod product;

use welds::WeldsModel;

#[derive(Debug, sqlx::FromRow, WeldsModel)]
#[welds(db(Mssql))]
#[welds(schema = "welds", table = "Thing1")]
pub struct Thing1 {
    #[welds(primary_key)]
    pub id: i32,
    pub value: String,
}
