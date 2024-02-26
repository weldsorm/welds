use welds::prelude::*;

/// Define a struct the maps to the products table in the databases
#[derive(Debug, WeldsModel)]
#[welds(db(Sqlite))]
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
#[welds(db(Sqlite))]
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

    // Create some data to play with
    create_data(client).await?;

    // We are writing a SELECT that pulls out only the name and price columns from Product
    // along with the sell_price from order.
    let q = Product::all()
        .where_col(|p| p.active.equal(true))
        .select(|p| p.name)
        .select(|p| p.price)
        .left_join(|p| p.orders, Order::select(|o| o.sell_price))
        .order_by_desc(|p| p.price)
        .limit(20);

    println!("SQL: {}", q.to_sql(client.syntax()));
    let mut rows = q.run(client).await?;

    // Simple basic struct to put our data in
    #[derive(Debug)]
    struct View {
        name: String,
        price: Option<f32>,
        sell_price: Option<f32>,
    }

    // Making a closure to describe how to read in the row
    // nice so that we can use the "?" operator when reading the row
    let from_row = |row: welds::Row| {
        let r: Result<View, welds::WeldsError> = Ok(View {
            name: row.get("name")?,
            price: row.get("price")?,
            sell_price: row.get("sell_price")?,
        });
        r
    };

    // Pull the data out of the rows into whatever thing you want
    let data: Result<Vec<View>, _> = rows.drain(..).map(from_row).collect();
    let data = data?;

    //print
    data.iter().for_each(|row| {
        println!("Row: {:?}", row);
    });

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
            sell_price: Some((i as f32) + 0.5),
        })
        .collect();
    welds::query::insert::bulk_insert(conn, &orders).await?;

    Ok(())
}
