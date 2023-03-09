use welds_core::query::clause::ClauseAdder;
use welds_core::query::clause::{Basic, Numeric};
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
}

impl Default for ProductSchema {
    fn default() -> Self {
        Self {
            id: Numeric::new("ID"),
            name: Basic::new("name"),
        }
    }
}

impl TableInfo for ProductSchema {
    fn identifier() -> &'static str {
        "welds.products"
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
        ]
    }
}

impl Product {
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
