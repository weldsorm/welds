use welds_core::query::clause::QueryBuilderAdder;
use welds_core::query::clause::{Clause, ClauseOpt};
use welds_core::query::optional::Optional;
use welds_core::table::TableInfo;

/*
 * NOTE: You shouldn't be writing Models by hand.
 * use the welds cli to generate models
 * The this model is here for the purpose of testing core itself
 * */

type Select<'args> = welds_core::query::select::SelectBuilder<'args, Product, ProductSchema>;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Product {
    pub product_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price1: Option<f32>,
    pub price2: Option<f64>,
    //pub price3: Option<sqlx::postgres::types::PgMoney>,
    //pub barcode: Option<Vec<u8>>,
    pub active: Option<bool>,
}

pub struct ProductSchema {
    pub id: Clause<i32>,
    pub name: Clause<String>,
    pub description: ClauseOpt<Optional<String>>,
    pub price1: ClauseOpt<Optional<f32>>,
    pub price2: ClauseOpt<Optional<f64>>,
    //pub price3: ClauseOpt<Optional<sqlx::postgres::types::PgMoney>>,
    //pub barcode: ClauseOpt<Optional<Vec<u8>>>,
    pub active: ClauseOpt<Optional<bool>>,
}

impl Default for ProductSchema {
    fn default() -> Self {
        Self {
            id: Clause::new("id"),
            name: Clause::new("name"),
            description: ClauseOpt::new("Description"),
            price1: ClauseOpt::new("price1"),
            price2: ClauseOpt::new("price2"),
            //price3: ClauseOpt::new("price3"),
            //barcode: ClauseOpt::new("barcode"),
            active: ClauseOpt::new("active"),
        }
    }
}

impl TableInfo for ProductSchema {
    fn identifier() -> &'static str {
        "products"
    }
    fn columns() -> &'static [&'static str] {
        &[
            "product_id",
            "name",
            "Description",
            "price1",
            "price2",
            //"price3",
            //"barcode",
            "active",
        ]
    }
}

impl Product {
    pub fn all<'args>() -> Select<'args> {
        Select::new()
    }
    pub fn where_col<'args>(
        lam: impl Fn(ProductSchema) -> Box<dyn QueryBuilderAdder<'args>>,
    ) -> Select<'args> {
        let select = Select::new();
        select.where_col(lam)
    }
}

#[test]
fn should_be_able_to_read_all_products() {
    async_std::task::block_on(async {
        let conn = testlib::postgres::conn().await.unwrap();
        let pool: welds_core::database::Pool = conn.into();
        let all = Product::all().run(&pool).await.unwrap();
        assert_eq!(all.len(), 6, "Unexpected number of rows returned");
    })
}

#[test]
fn should_be_able_to_filter_on_equal() {
    async_std::task::block_on(async {
        let conn = testlib::postgres::conn().await.unwrap();
        let pool: welds_core::database::Pool = conn.into();
        let just_horse = Product::where_col(|x| x.price1.equal(1.10))
            .run(&pool)
            .await
            .unwrap();
        assert_eq!(
            just_horse.len(),
            1,
            "Expected to only find the horse in the test data"
        );
    })
}
