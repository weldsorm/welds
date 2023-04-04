use sqlx::Executor;
use welds::state::DbState;
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
    // Build an in memory DB with a schema (Product Table, Orders Table)
    let pool = sqlx::SqlitePool::connect("sqlite::memory:").await?;
    let schema = include_str!("../../tests/testlib/databases/sqlite/01_create_tables.sql");
    pool.clone().execute(schema).await?;
    // Create and update a Product
    let product = create_and_update_products(&pool).await?;
    // Create a bunch of orders
    create_orders(&product, &pool).await?;
    // Count the Orders Using the Product
    chain_query_together(&pool).await?;

    Ok(())
}

async fn create_and_update_products(
    pool: &sqlx::SqlitePool,
) -> Result<DbState<Product>, Box<dyn std::error::Error>> {
    let mut transaction = pool.begin().await?;

    // create the product
    let mut p = Product::new();
    p.name = "Girl Scout Cookies".to_owned();
    p.active = true;
    p.save(&mut transaction).await?;
    println!("Product Created: {:?}", p);

    // update the product
    p.description = Some("Yummy !!!".to_owned());
    p.save(&mut transaction).await?;
    println!("Product Updated: {:?}", p);

    // Don't forget to commit :)
    // default is to rollback
    transaction.commit().await?;

    Ok(p)
}

async fn create_orders(
    product: &Product,
    pool: &sqlx::SqlitePool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = pool.acquire().await?;
    for _ in 0..100 {
        let mut o = Order::new();
        o.product_id = Some(product.id);
        o.sell_price = Some(3.50);
        o.save(&mut conn).await?;
    }
    println!("Orders Created");
    Ok(())
}

async fn chain_query_together(pool: &sqlx::SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = pool.acquire().await?;

    // Start from a product and ending on its orders
    let mut order_query = Product::all()
        .order_by_asc(|p| p.id)
        .limit(1)
        .map_query(|p| p.orders)
        .where_col(|x| x.id.lte(2));

    let sql = order_query.to_sql();
    let orders = order_query.run(&mut conn).await?;
    let count_in_sql = order_query.count(&mut conn).await?;

    println!("Some Orders SQL: {}", sql);
    println!("Some Orders {}: {:?}", count_in_sql, orders);

    Ok(())
}
