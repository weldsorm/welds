use welds::prelude::*;

// This struct doesn't have a table in the DB
#[derive(Debug, WeldsModel)]
#[welds(table = "i_dont_exist")]
pub struct NotInDb {
    #[welds(primary_key)]
    pub id: i32,
}

#[derive(Debug, WeldsModel)]
#[welds(table = "Orders")]
pub struct Order {
    #[welds(primary_key)]
    // null in db
    pub id: i32,
    // null int in db
    pub product_id: String,
    /*
     * these are removed to show you what happens when they are missing
    pub price: Option<f32>,
    pub code: Option<String>,
    pub product_id2: Option<i32>,
     */
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection_string = "sqlite::memory:";
    let client = welds::connections::connect(connection_string).await?;

    // Build an in memory DB with a schema (Product Table, Orders Table)
    let schema = include_str!("../../tests/testlib/databases/sqlite/01_create_tables.sql");
    client.execute(schema, &[]).await?;

    // the NotInDb struct will only report back that is isn't in the database
    println!();
    let diff = welds::check::schema::<NotInDb>(client.as_ref()).await?;
    for d in diff {
        println!("{}", d);
    }

    // Get all the things that are different from the Order struct and the order table in the DB
    let diff = welds::check::schema::<Order>(client.as_ref()).await?;
    for d in &diff {
        println!("{}", d);
    }

    println!();
    // let look over just the columns that have changed types
    // on the Orders table/struct
    println!("Orders Changed Columns:");
    for d in &diff {
        if let Some(changed) = d.kind.as_changed() {
            if changed.type_changed() {
                println!("{}", changed);
            }
        }
    }

    Ok(())
}
