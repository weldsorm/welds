use sqlx::postgres::types::PgMoney;
use welds_core::WeldsModel;

/*
 * NOTE: You shouldn't be writing Models by hand.
 * use the welds cli to generate models
 * The this model is here for the purpose of testing core itself
 * */

#[derive(Debug, sqlx::FromRow, WeldsModel)]
#[welds(db(Postgres))]
#[welds(table = "products")]
pub struct Product {
    #[sqlx(rename = "product_id")]
    #[welds(primary_key)]
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price1: Option<f32>,
    pub price2: Option<f64>,
    pub price3: Option<PgMoney>,
    pub barcode: Option<Vec<u8>>,
    pub active: Option<bool>,
}

impl welds_core::relations::HasRelations for Product {
    type Relation = ProductRelation;
}

use welds_core::relations::*;
pub struct ProductRelation {
    pub orders: HasMany<super::order::Order>,
}

impl Default for ProductRelation {
    fn default() -> Self {
        Self {
            orders: HasMany::using("product_id"),
        }
    }
}

impl welds_core::table::UniqueIdentifier<sqlx::Postgres> for Product {
    fn id_column() -> welds_core::table::Column {
        welds_core::table::Column::new::<sqlx::Postgres, i32>("product_id")
    }
}
