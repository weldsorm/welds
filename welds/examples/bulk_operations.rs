use welds::prelude::*;

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
    let connection_string = "sqlite::memory:";
    let client = welds::connections::connect(connection_string).await?;
    let client = client.as_ref();

    // Build an in memory DB with a schema (Product Table, Orders Table)
    let schema = include_str!("../../tests/testlib/databases/sqlite/01_create_tables.sql");
    client.execute(schema, &[]).await?;

    // Create a bunch of products to play with
    create_products(client).await?;
    println!();

    // Bulk update Orders and assign to first product
    Order::all().set(|x| x.product_id, 1).run(client).await?;
    //verify Change
    let order_count = Product::where_col(|p| p.id.equal(1))
        .map_query(|p| p.orders)
        .count(client)
        .await?;
    println!("Product:1 orders: {}", order_count);

    // Bulk update Orders selecting through the product with order filter
    Product::where_col(|p| p.id.equal(1))
        .map_query(|p| p.orders)
        .where_col(|o| o.id.gt(400))
        .set(|o| o.sell_price, 3.50)
        .run(client)
        .await?;
    //verify change
    let order_count = Order::where_col(|o| o.sell_price.gt(3.0))
        .count(client)
        .await?;
    println!("Updated orders: {}", order_count);

    // Bulk Delete
    let query = Product::where_col(|p| p.id.equal(1))
        .map_query(|p| p.orders)
        .where_col(|o| o.sell_price.equal(None));
    println!("To Delete orders: {}", query.count(client).await?);
    query.delete(client).await?;

    //verify deletion
    let order_count = Order::where_col(|o| o.sell_price.not_equal(None))
        .count(client)
        .await?;
    println!("Remaining orders: {}", order_count);

    Ok(())
}

async fn create_products(client: &dyn Client) -> Result<(), Box<dyn std::error::Error>> {
    let products: Vec<_> = (0..1000)
        .map(|i| Product {
            id: 0,
            name: format!("product #{}", i),
            description: None,
            price: None,
            active: true,
        })
        .collect();
    println!("Products::new()");
    welds::query::insert::bulk_insert(client, &products).await?;
    println!("Products::inserted()");

    let orders: Vec<_> = (0..1000)
        .map(|i| Order {
            id: 0,
            product_id: Some(i + 1),
            sell_price: None,
        })
        .collect();
    welds::query::insert::bulk_insert(client, &orders).await?;

    let total_p = Product::all().count(client).await?;
    let total_o = Order::all().count(client).await?;
    println!();
    println!("Created Products: {}", total_p);
    println!("Created Products: {}", total_o);
    Ok(())
}
