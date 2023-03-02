use welds_core::query::clause::ClauseAdder;
use welds_core::query::clause::{Basic, BasicOpt, Numeric, NumericOpt};
use welds_core::query::optional::Optional;
use welds_core::query::select::SelectBuilder;
use welds_core::table::TableInfo;

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
}

pub struct ProductSchema {
    pub id: Numeric<i32>,
    pub name: Basic<String>,
}

impl Default for ProductSchema {
    fn default() -> Self {
        Self {
            id: Numeric::new("id"),
            name: Basic::new("name"),
        }
    }
}

impl TableInfo for ProductSchema {
    fn identifier() -> &'static str {
        "welds.products"
    }
    fn columns() -> &'static [&'static str] {
        &["ID", "name"]
    }
}

impl Product {
    pub fn all<'args, DB>() -> SelectBuilder<'args, Self, ProductSchema, DB>
    where
        DB: sqlx::Database,
        Self: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row>,
    {
        SelectBuilder::new()
    }
    pub fn where_col<'args, DB>(
        lam: impl Fn(ProductSchema) -> Box<dyn ClauseAdder<'args, DB>>,
    ) -> SelectBuilder<'args, Self, ProductSchema, DB>
    where
        DB: sqlx::Database,
        Self: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row>,
    {
        let select = SelectBuilder::new();
        select.where_col(lam)
    }
}
