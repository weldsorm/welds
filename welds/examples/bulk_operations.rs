use sqlx::{Executor, Sqlite};
use welds::connection::Connection;
use welds::WeldsModel;

/// Define a struct the maps to the products table in the databases
#[derive(Debug, sqlx::FromRow, WeldsModel)]
#[welds(db(Sqlite))]
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
#[welds(db(Sqlite))]
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
    //println!();

    // Bulk update Orders and assign to first product
    Order::all().set(|x| x.product_id, 1).run(&pool).await?;
    //verify Change
    let order_count = Product::where_col(|p| p.id.equal(1))
        .map_query(|p| p.orders)
        .count(&pool)
        .await?;
    println!("Product:1 orders: {}", order_count);

    // Bulk update Orders selecting through the product with order filter
    Product::where_col(|p| p.id.equal(1))
        .map_query(|p| p.orders)
        .where_col(|o| o.id.gt(400))
        .set(|o| o.sell_price, 3.50)
        .run(&pool)
        .await?;
    //verify change
    let order_count = Order::where_col(|o| o.sell_price.gt(3.0))
        .count(&pool)
        .await?;
    println!("Updated orders: {}", order_count);

    // Bulk Delete
    let query = Product::where_col(|p| p.id.equal(1))
        .map_query(|p| p.orders)
        .where_col(|o| o.sell_price.equal(None));
    println!("To Delete orders: {}", query.count(&pool).await?);
    query.delete(&pool).await?;

    //verify deletion
    let order_count = Order::where_col(|o| o.sell_price.not_equal(None))
        .count(&pool)
        .await?;
    println!("Remaining orders: {}", order_count);

    Ok(())
}

async fn create_products(conn: &impl Connection<Sqlite>) -> Result<(), Box<dyn std::error::Error>> {
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

    let orders: Vec<_> = (0..1000)
        .map(|i| Order {
            id: 0,
            product_id: Some(i + 1),
            sell_price: None,
        })
        .collect();
    welds::query::insert::bulk_insert(conn, &orders).await?;

    let total_p = Product::all().count(conn).await?;
    let total_o = Order::all().count(conn).await?;
    println!();
    println!("Created Products: {}", total_p);
    println!("Created Products: {}", total_o);
    Ok(())
}
