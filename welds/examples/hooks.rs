use welds::prelude::*;

/// Define a struct the maps to the products table in the databases
#[derive(Debug, WeldsModel)]
#[welds(table = "products")]
// Wiring up a bunch of hooks for when this model touches the database.
#[welds(BeforeCreate(before_create))]
#[welds(AfterCreate(after_create, async = true))]
#[welds(AfterCreate(after_create_second))]
#[welds(BeforeUpdate(before_update))]
#[welds(AfterUpdate(after_update))]
#[welds(BeforeDelete(before_delete))]
#[welds(AfterDelete(after_delete))]

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

// *******************************************************************
// Note: welds::errors::WeldsError does support Anyhow errors
// This way your own types can be passed through
// *******************************************************************

fn before_create(product: &mut Product) -> welds::errors::Result<()> {
    println!("Before Create: {:?}", product);
    Ok(())
}

// Example async callback
async fn after_create(product: &Product) {
    print_message(product).await;
}

fn after_create_second(product: &Product) {
    // they run in the order they are defined
    println!("After Create2: {:?}", product);
}

fn before_update(product: &mut Product) -> welds::errors::Result<()> {
    println!("Before Update: {:?}", product);
    Ok(())
}

fn after_update(product: &Product) {
    println!("After Update: {:?}", product);
}

fn before_delete(product: &Product) -> welds::errors::Result<()> {
    eprintln!("Before Delete: {:?}", product);
    Ok(())
}

fn after_delete(product: &Product) {
    println!("After Delete: {:?}", product);
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let connection_string = "sqlite::memory:";
    let client = welds::connections::connect(connection_string).await?;

    // Build an in memory DB with a schema (Product Table, Orders Table)
    let schema = include_str!("../../tests/testlib/databases/sqlite/01_create_tables.sql");
    client.execute(schema, &[]).await?;

    let mut product = new_product();
    product.save(&client).await?;

    product.name = "test".to_string();
    product.save(&client).await?;

    product.delete(&client).await?;

    eprintln!("Done");

    Ok(())
}

fn new_product() -> DbState<Product> {
    DbState::new_uncreated(Product {
        id: 0,
        name: "Cookie".to_owned(),
        description: None,
        price: Some(3.15),
        active: true,
    })
}

async fn print_message(product: &Product) {
    println!("After Create: {:?}", product);
}
