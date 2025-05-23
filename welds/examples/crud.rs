use welds::{Syntax, WeldsError, prelude::*};

/// Define a struct the maps to the products table in the databases
#[derive(Debug, WeldsModel)]
#[welds(table = "products")]
#[welds(HasMany(orders, Order, "product_id"))]
pub struct Product {
    #[welds(primary_key)]
    #[welds(rename = "product_id")]
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    #[welds(rename = "price1")]
    pub price: Option<f32>,
    pub active: bool,
}

/// Define a Struct the maps to the Orders table in the databases
#[derive(Debug, WeldsModel)]
#[welds(table = "orders")]
#[welds(BelongsTo(product, Product, "product_id"))]
pub struct Order {
    #[welds(primary_key)]
    pub id: i32,
    pub product_id: Option<i32>,
    #[welds(rename = "price")]
    pub sell_price: Option<f32>,
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let connection_string = "sqlite::memory:";
    let client = welds::connections::connect(connection_string).await?;

    // Build an in memory DB with a schema (Product Table, Orders Table)
    let schema = include_str!("../../tests/testlib/databases/sqlite/01_create_tables.sql");
    client.execute(schema, &[]).await?;

    // Create and update a Product
    let trans = client.begin().await?;
    let product = create_and_update_products(&trans).await?;
    trans.commit().await?;

    // Create a bunch of orders
    create_orders(&product, &client).await?;

    // Select the Orders Using the Product
    chain_query_together(&client).await?;

    // Filter Orders using relationships from other tables
    filter_order_using_relationships(&client).await?;

    // Delete Some Stuff
    let product2 = create_and_update_products(&client).await?;
    delete_the_product(&client, product2.id).await?;

    let _ = Product::all().set(|x| x.description, "".to_string());

    Ok(())
}

async fn create_and_update_products(client: &impl Client) -> Result<DbState<Product>, WeldsError> {
    // create the product
    let mut p = Product::new();
    p.name = "Girl Scout Cookies".to_owned();
    p.active = true;
    p.save(client).await?;
    println!("Product Created: {:?}", p);

    // update the product
    p.description = Some("Yummy !!!".to_owned());
    p.save(client).await?;
    println!("Product Updated: {:?}", p);
    Ok(p)
}

async fn create_orders(product: &Product, conn: &impl Client) -> Result<(), WeldsError> {
    for _ in 0..100 {
        let mut o = Order::new();
        o.product_id = Some(product.id);
        o.sell_price = Some(3.50);
        o.save(conn).await?;
    }
    let total = Order::all().count(conn).await?;
    println!();
    println!("Orders Created: {}", total);
    Ok(())
}

async fn chain_query_together(conn: &impl Client) -> Result<(), WeldsError> {
    // Start from a product and ending on its orders
    let order_query = Product::all()
        .order_by_asc(|p| p.id)
        .limit(1)
        .map_query(|p| p.orders)
        .where_col(|x| x.id.lte(2));

    let sql = order_query.to_sql(Syntax::Sqlite);

    let orders = order_query.run(conn).await?;

    println!();
    println!("Some Orders SQL: {}", sql);
    println!("Some Orders: {:?}", orders);

    Ok(())
}

async fn filter_order_using_relationships(
    conn: &impl Client,
) -> Result<(), Box<dyn std::error::Error>> {
    // NOTE: this is an un-executed query.
    let product_query = Product::where_col(|p| p.name.like("%Cookie%"));

    // select all the orders, where order match the product query
    let orders = Order::all()
        .where_relation(|o| o.product, product_query)
        .run(conn)
        .await?;

    println!();
    println!("Found More Orders: {}", orders.len());
    Ok(())
}

async fn delete_the_product(
    conn: &impl Client,
    product_id: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut product = Product::find_by_id(conn, product_id).await?.unwrap();
    product.delete(conn).await?;
    let count = Product::all().count(conn).await?;

    println!();
    println!("DELETE: {:?}", product);
    println!("NEW COUNT: {}", count);
    Ok(())
}
