use welds::WeldsModel;

/*
 * NOTE: You shouldn't be writing Models by hand.
 * use the welds cli to generate models
 * The this model is here for the purpose of testing core itself
 * */

#[derive(Debug, sqlx::FromRow, WeldsModel)]
#[welds(db(Mssql))]
#[welds(schema = "welds", table = "products")]
pub struct Product {
    #[welds(primary_key)]
    #[sqlx(rename = "ID")]
    pub id: i32,
    pub name: String,
}
