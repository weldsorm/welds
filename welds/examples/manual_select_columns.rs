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
    pub sale_price: Option<f32>,
}

/// Define a struct that we want to put the combined selected data into
/// NOTE: This struct doesn't have a table linked to it.
#[derive(Debug, WeldsModel)]
pub struct ProductSale {
    pub product_name: String,
    pub sale_price: Option<f32>,
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection_string = "sqlite::memory:";
    let client = welds::connections::connect(connection_string).await?;
    let client = client.as_ref();

    // Build an in memory DB with a schema (Product Table, Orders Table)
    let schema = include_str!("../../tests/testlib/databases/sqlite/01_create_tables.sql");
    client.execute(schema, &[]).await?;

    // Create some data to play with
    create_data(client).await?;

    // For this example lets select out only the product names
    // and the price it sold for.
    //
    // We will pull the price it sold for off the orders table,
    // and the name of the product off of the product table

    //start by selecting the price it was sold at.
    let q = Order::select(|o| o.sale_price)
        // now join to get the product name
        .join(
            |o| o.product,
            // selecting out the name column and renaming it
            Product::select_as(|p| p.name, "product_name"),
        );

    // Run the query and "collect" the rows out into your struct
    let product_sales: Vec<ProductSale> = q.run(client).await?.collect_into()?;

    //Print the selected columns
    for product_sale in product_sales {
        println!("Product Sale: {:?}", product_sale);
    }

    Ok(())
}

// Just a little helper function to create some data to play with
async fn create_data(conn: &dyn Client) -> Result<(), Box<dyn std::error::Error>> {
    // Create some product records
    let products: Vec<_> = (0..1000)
        .map(|i| Product {
            id: 0,
            name: format!("product #{}", i),
            description: None,
            price: Some(i as f32),
            active: true,
        })
        .collect();
    welds::query::insert::bulk_insert(conn, &products).await?;

    // Create some order records
    let orders: Vec<_> = (0..500)
        .map(|i| Order {
            id: 0,
            product_id: Some((i + 1) * 2), //skip every other product
            sale_price: Some((i as f32) + 0.5),
        })
        .collect();
    welds::query::insert::bulk_insert(conn, &orders).await?;

    Ok(())
}
