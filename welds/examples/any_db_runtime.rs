use sqlx::database::HasArguments;
use sqlx::Executor;
use sqlx::{FromRow, IntoArguments};
use welds::connection::Connection;
use welds::connection::Database;
use welds::table::{HasSchema, TableColumns, WriteToArgs};
use welds::WeldsModel;

// ***************************************************************************************************
// This is an example of how you can write code to support multiple databases at runtime.
// If you are willing to support database switching at compile time, this code becomes MUCH cleaner.
// check out the any_db_compiletime for an example of that.
// ***************************************************************************************************

/// Define a struct the maps to the products table in the databases
#[derive(Debug, sqlx::FromRow, WeldsModel)]
#[welds(db(Sqlite, Postgres))]
#[welds(table = "products")]
#[welds(HasMany(orders, Order, "product_id"))]
pub struct Product {
    #[welds(primary_key)]
    #[sqlx(rename = "product_id")]
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    #[sqlx(rename = "price1")]
    pub price: Option<f32>,
    pub active: bool,
}

/// Define a Struct the maps to the Orders table in the databases
#[derive(Debug, sqlx::FromRow, WeldsModel)]
#[welds(db(Sqlite, Postgres))]
#[welds(table = "orders")]
#[welds(BelongsTo(product, Product, "product_id"))]
pub struct Order {
    #[welds(primary_key)]
    pub id: i32,
    pub product_id: Option<i32>,
    #[sqlx(rename = "price")]
    pub sell_price: Option<f32>,
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection_string = "sqlite::memory:";
    let pool = welds::connection::connect_sqlite(connection_string).await?;

    // Build an in memory DB with a schema (Product Table, Orders Table)
    let schema = include_str!("../../tests/testlib/databases/sqlite/01_create_tables.sql");
    pool.as_sqlx_pool().execute(schema).await?;

    // Create a bunch of products to play with
    create_products(&pool).await?;

    // get the products from the database
    get_products(&pool).await?;

    Ok(())
}

async fn get_products<'c, 'args, C, DB>(conn: &'c C) -> Result<(), Box<dyn std::error::Error>>
where
    'c: 'args,
    C: Connection<DB>,
    DB: Database,
    <DB as HasArguments<'c>>::Arguments: IntoArguments<'args, DB>,
    Product: WriteToArgs<DB> + for<'fr> FromRow<'fr, DB::Row>,
    <Product as HasSchema>::Schema: TableColumns<DB>,
{
    let all_products = Product::all().run(conn).await?;
    println!();
    println!("Created Products: {}", all_products.len());
    Ok(())
}

async fn create_products<'c, 'args, C, DB>(conn: &'c C) -> Result<(), Box<dyn std::error::Error>>
where
    'c: 'args,
    C: Connection<DB>,
    DB: Database,
    <DB as HasArguments<'c>>::Arguments: IntoArguments<'args, DB>,
    Product: WriteToArgs<DB> + for<'fr> FromRow<'fr, DB::Row>,
    <Product as HasSchema>::Schema: TableColumns<DB>,
{
    let products: Vec<_> = (0..1000)
        .map(|i| Product {
            id: 0,
            name: format!("product #{}", i),
            description: None,
            price: None,
            active: true,
        })
        .collect();
    welds::query::insert::bulk_insert(conn, &products).await?;

    Ok(())
}
