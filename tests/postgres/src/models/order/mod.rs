use welds_core::WeldsModel;

/*
 * NOTE: You shouldn't be writing Models by hand.
 * use the welds cli to generate models
 * The this model is here for the purpose of testing core itself
 * */

#[derive(Debug, sqlx::FromRow, WeldsModel)]
#[welds(db(Postgres))]
#[welds(table = "Orders")]
#[welds(BelongsTo(product, super::product::Product, "product_id"))]
pub struct Order {
    #[welds(primary_key)]
    pub id: i64,
    pub product_id: Option<i32>,
    pub quantity: Option<i16>,
    pub code: Option<String>,
    #[sqlx(rename = "SoldFor")]
    pub sold_for: Option<f64>,
}
