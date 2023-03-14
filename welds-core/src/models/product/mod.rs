use crate::query::clause::ClauseAdder;
use crate::query::clause::{Basic, BasicOpt, Numeric, NumericOpt};
use crate::query::optional::Optional;
use crate::query::select::SelectBuilder;
use crate::table::{Column, HasSchema, TableColumns, TableInfo, WriteToArgs};

/*
 * NOTE: You shouldn't be writing Models by hand.
 * use the welds cli to generate models
 * The this model is here for the purpose of testing core itself
 * */

#[derive(Debug, sqlx::FromRow)]
pub struct Product {
    #[sqlx(rename = "product_id")]
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price1: Option<f32>,
    pub price2: Option<f64>,
    pub active: Option<bool>,
}

impl HasSchema for Product {
    type Schema = ProductSchema;
}

//impl WriteToArgs<sqlx::Sqlite> for Product {
//    fn bind<'args>(
//        &self,
//        column: &str,
//        args: &mut <sqlx::Sqlite as sqlx::database::HasArguments<'args>>::Arguments,
//    ) -> Result<(), welds_core::errors::WeldsError> {
//        use sqlx::Arguments;
//        match column {
//            "product_id" => args.add(&self.id),
//            "name" => args.add(&self.name),
//            "description" => args.add(&self.description),
//            "price1" => args.add(&self.price1),
//            "price2" => args.add(&self.price2),
//            "active" => args.add(&self.active),
//            _ => {
//                return Err(welds_core::errors::WeldsError::MissingDbColumn(
//                    column.to_owned(),
//                ))
//            }
//        }
//        Ok(())
//    }
//}

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
            id: Numeric::new("product_id"),
            name: Basic::new("name"),
            description: BasicOpt::new("description"),
            price1: NumericOpt::new("price1"),
            price2: NumericOpt::new("price2"),
            active: BasicOpt::new("active"),
        }
    }
}

impl TableInfo for ProductSchema {
    fn identifier() -> &'static str {
        "Products"
    }
}

impl TableColumns<sqlx::Sqlite> for ProductSchema {
    fn primary_keys() -> Vec<Column> {
        type DB = sqlx::Sqlite;
        vec![Column::new::<DB, i32>("product_id")]
    }
    fn columns() -> Vec<Column> {
        type DB = sqlx::Sqlite;
        vec![
            Column::new::<DB, i32>("product_id"),
            Column::new::<DB, String>("name"),
            Column::new::<DB, Option<String>>("description"),
            Column::new::<DB, Option<f32>>("price1"),
            Column::new::<DB, Option<f64>>("price2"),
            Column::new::<DB, Option<bool>>("active"),
        ]
    }
}

impl Product {
    pub fn new() -> welds_core::state::DbState<Self> {
        welds_core::state::DbState::new_uncreated(Self {
            id: Default::default(),
            name: Default::default(),
            description: Default::default(),
            price1: Default::default(),
            price2: Default::default(),
            active: Default::default(),
        })
    }

    pub fn all<'args, DB>() -> SelectBuilder<'args, Self, DB>
    where
        DB: sqlx::Database,
        ProductSchema: TableColumns<DB>,
        Self: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row>,
    {
        SelectBuilder::new()
    }

    pub fn where_col<'args, DB>(
        lam: impl Fn(ProductSchema) -> Box<dyn ClauseAdder<'args, DB>>,
    ) -> SelectBuilder<'args, Self, DB>
    where
        DB: sqlx::Database,
        ProductSchema: TableColumns<DB>,
        Self: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row>,
    {
        let select = SelectBuilder::new();
        select.where_col(lam)
    }
}
