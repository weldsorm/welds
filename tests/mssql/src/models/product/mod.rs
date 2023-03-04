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
    #[sqlx(rename = "ID")]
    pub id: i32,
    #[sqlx(rename = "name")]
    pub name: String,
    #[sqlx(rename = "Description")]
    pub description: Option<String>,
    #[sqlx(rename = "price1")]
    pub price1: Option<f32>,
    #[sqlx(rename = "price2")]
    pub price2: Option<f32>,
    #[sqlx(rename = "active")]
    pub active: Option<i32>,
}

pub struct ProductSchema {
    pub id: Numeric<i32>,
    pub name: Basic<String>,
    pub description: BasicOpt<Optional<String>>,
    pub price1: NumericOpt<Optional<f32>>,
    pub price2: NumericOpt<Optional<f64>>,
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
            active: BasicOpt::new("active"),
        }
    }
}

impl TableInfo for ProductSchema {
    fn identifier() -> &'static str {
        "welds.Products"
    }
}

impl TableColumns<sqlx::Mssql> for ProductSchema {
    fn columns() -> Vec<Column> {
        type DB = sqlx::Mssql;
        vec![
            Column::new::<DB, i32>("ID"),
            Column::new::<DB, String>("name"),
            Column::new::<DB, Option<String>>("Description"),
            Column::new::<DB, Option<f32>>("price1"),
            Column::new::<DB, Option<f32>>("price2"),
            Column::new::<DB, Option<i32>>("active"),
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
