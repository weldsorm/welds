use sqlx::postgres::types::PgMoney;
use welds_core::query::clause::ClauseAdder;
use welds_core::query::clause::{Basic, BasicOpt, Numeric, NumericOpt};
use welds_core::query::optional::Optional;
use welds_core::query::select::SelectBuilder;
use welds_core::table::{Column, TableColumns, TableInfo};

/*
 * NOTE: You shouldn't be writing Models by hand.
 * use the welds cli to generate models
 * The this model is here for the purpose of testing core itself
 * */

#[derive(Default, Debug, Clone, sqlx::FromRow)]
pub struct Product {
    pub product_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price1: Option<f32>,
    pub price2: Option<f64>,
    pub price3: Option<PgMoney>,
    pub barcode: Option<Vec<u8>>,
    pub active: Option<bool>,
}

pub struct ProductSchema {
    pub id: Numeric<i32>,
    pub name: Basic<String>,
    pub description: BasicOpt<Optional<String>>,
    pub price1: NumericOpt<Optional<f32>>,
    pub price2: NumericOpt<Optional<f64>>,
    pub price3: NumericOpt<Optional<PgMoney>>,
    pub barcode: BasicOpt<Optional<Vec<u8>>>,
    pub active: BasicOpt<Optional<bool>>,
}

impl Default for ProductSchema {
    fn default() -> Self {
        Self {
            id: Numeric::new("id"),
            name: Basic::new("name"),
            description: BasicOpt::new("Description"),
            price1: NumericOpt::new("price1"),
            price2: NumericOpt::new("price2"),
            price3: NumericOpt::new("price3"),
            barcode: BasicOpt::new("barcode"),
            active: BasicOpt::new("active"),
        }
    }
}

impl TableInfo for ProductSchema {
    fn identifier() -> &'static str {
        "products"
    }
}

impl TableColumns<sqlx::Postgres> for ProductSchema {
    fn columns() -> Vec<Column> {
        type DB = sqlx::Postgres;
        vec![
            Column::new::<DB, i32>("product_id"),
            Column::new::<DB, String>("name"),
            Column::new::<DB, Option<String>>("Description"),
            Column::new::<DB, Option<f32>>("price1"),
            Column::new::<DB, Option<f64>>("price2"),
            Column::new::<DB, Option<PgMoney>>("price3"),
            Column::new::<DB, Option<Vec<u8>>>("barcode"),
            Column::new::<DB, Option<bool>>("active"),
        ]
    }
}

impl Product {
    pub fn all<'args, DB>() -> SelectBuilder<'args, Self, ProductSchema, DB>
    where
        DB: sqlx::Database,
        ProductSchema: TableColumns<DB>,
        Self: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row>,
    {
        SelectBuilder::new()
    }
    pub fn where_col<'args, DB>(
        lam: impl Fn(ProductSchema) -> Box<dyn ClauseAdder<'args, DB>>,
    ) -> SelectBuilder<'args, Self, ProductSchema, DB>
    where
        DB: sqlx::Database,
        ProductSchema: TableColumns<DB>,
        Self: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row>,
    {
        let select = SelectBuilder::new();
        select.where_col(lam)
    }
}
