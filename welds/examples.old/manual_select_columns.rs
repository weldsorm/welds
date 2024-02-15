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

    // Create some data to play with
    create_data(&pool).await?;

    // We are writing a SELECT that pulls out only the name and price columns from Product
    // along with the sell_price from order.
    let q = Product::all()
        .where_col(|p| p.active.equal(true))
        .select(|p| p.name)
        .select(|p| p.price)
        .left_join(|p| p.orders, Order::select(|o| o.sell_price))
        .order_by_desc(|p| p.price)
        .limit(10);

    println!("SQL: {}", q.to_sql());
    let sqlx_rows = q.run(&pool).await?;

    // The select returns `sqlx::Row`.
    // This allows us to easily map into any struct that implements sqlx::FromRow.
    //
    // Alternatively you could pull the data from each column using sqlx::Row and get::<T>(x)
    use sqlx::FromRow;
    #[derive(FromRow, Debug)]
    struct View {
        name: String,
        price: Option<f32>,
        sell_price: Option<f32>,
    }

    // Pull the data out of the sqlx rows into whatever thing you want
    let data: Vec<_> = sqlx_rows.iter().map(View::from_row).collect();

    //print
    data.iter().for_each(|row| {
        println!("Row: {:?}", row);
    });

    Ok(())
}

// Just a little helper function to create some data to play with
async fn create_data(conn: &impl Connection<Sqlite>) -> Result<(), Box<dyn std::error::Error>> {
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
