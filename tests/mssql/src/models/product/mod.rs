use welds_core::query::clause::ClauseAdder;
use welds_core::query::clause::{Basic, BasicOpt, Numeric, NumericOpt};
use welds_core::query::optional::Optional;
use welds_core::query::select::SelectBuilder;
use welds_core::table::{Column, HasSchema, TableColumns, TableInfo, WriteToArgs};

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

impl HasSchema for Product {
    type Schema = ProductSchema;
}

impl WriteToArgs<sqlx::Mssql> for Product {
    fn bind<'args>(
        &self,
        column: &str,
        args: &mut <sqlx::Mssql as sqlx::database::HasArguments<'args>>::Arguments,
    ) -> Result<(), welds_core::errors::WeldsError> {
        use sqlx::Arguments;
        match column {
            "ID" => args.add(&self.id),
            "name" => args.add(&self.name),
            "Description" => args.add(&self.description),
            "price1" => args.add(&self.price1),
            "price2" => args.add(&self.price2),
            "active" => args.add(&self.active),
            _ => {
                return Err(welds_core::errors::WeldsError::MissingDbColumn(
                    column.to_owned(),
                ))
            }
        }
        Ok(())
    }
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
            id: Numeric::new("ID"),
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
    fn primary_keys() -> Vec<Column> {
        type DB = sqlx::Mssql;
        vec![Column::new::<DB, i32>("product_id")]
    }
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
