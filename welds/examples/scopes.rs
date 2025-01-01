use welds::prelude::*;
use welds::query::builder::QueryBuilder;

/// Define a struct to use for this example.
#[derive(Debug, WeldsModel)]
#[welds(table = "products")]
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

// make a scope:
//
// In welds, scopes are just method extensions.
// You make a trait for your scope(s) and impl it for QueryBuilder

// first define what the scope will look like.
pub trait ProductScopes {
    fn ready_to_sell(self) -> Self;
}

// now impl the scope
impl ProductScopes for QueryBuilder<Product> {
    fn ready_to_sell(self) -> Self {
        self.where_col(|p| p.active.not_equal(false))
            .where_col(|p| p.price.not_equal(None))
            .where_col(|p| p.description.not_equal(None))
    }
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let connection_string = "sqlite::memory:";
    let client = welds::connections::sqlite::connect(connection_string).await?;

    // Build an in memory DB with a schema (Product Table, Orders Table)
    let schema = include_str!("../../tests/testlib/databases/sqlite/01_create_tables.sql");
    client.execute(schema, &[]).await?;

    // Create Products, Note: only one is "ready_to_sell"
    create_and_update_products(&client).await?;

    // make sure you include your scopes to use them :)
    // usually at the top of the file, here as a reminder for you
    use ProductScopes;

    let ready = Product::all()
        // note: they can be chained with regular queries
        .where_col(|x| x.id.gt(0))
        .ready_to_sell()
        .run(&client)
        .await?;

    println!("Ready To Sell: ");
    for product in &ready {
        println!("{:?}", product);
    }

    Ok(())
}

async fn create_and_update_products(
    client: &impl Client,
) -> welds::errors::Result<DbState<Product>> {
    // create first product
    let mut p = Product::new();
    p.name = "Girl Scout Cookies".to_owned();
    p.description = Some("chocolate chip".to_owned());
    p.price = Some(3.50);
    p.active = true;
    p.save(client).await?;
    println!("Product Created: {:?}", p);

    // create second product, not ready to sell
    let mut p2 = Product::new();
    p2.name = "Boy Scout Popcorn".to_owned();
    p2.save(client).await?;
    println!("Product Created: {:?}", p2);
    Ok(p)
}
