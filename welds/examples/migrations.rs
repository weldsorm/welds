use welds::errors::Result;
use welds::migrations::prelude::*;

#[async_std::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    // Connect and setup a DB for use to play with
    let client = welds::connections::sqlite::connect("sqlite::memory:").await?;

    // run all the migrations
    // This will skip over migrations that have already ran
    up(
        &client,
        &[
            create_peoples_table,
            create_addresses_table,
            add_address_to_people,
            rename_and_make_nullable,
        ],
    )
    .await?;
    println!("Migrate Up Complete");

    // lets rollback the last change
    let downed = down_last(&client).await?;
    println!("Migrate Down Complete");
    println!("Rollback: {}", downed.unwrap());

    Ok(())
}

// A simple migration to setup the peoples table.
fn create_peoples_table(_: &TableState) -> Result<MigrationStep> {
    let m = create_table("people")
        .id(|c| c("id", Type::Int))
        .column(|c| c("name", Type::String).create_unique_index());
    Ok(MigrationStep::new("create_peoples_table", m))
}

// A simple migration to setup the addresses table.
fn create_addresses_table(_: &TableState) -> Result<MigrationStep> {
    let m = create_table("addresses")
        .id(|c| c("id", Type::Int))
        .column(|c| c("name", Type::String).create_unique_index())
        .column(|c| c("finger_count", Type::IntSmall));
    Ok(MigrationStep::new("create_addresses_table", m))
}

// Let add a column to people to wire the two together
fn add_address_to_people(state: &TableState) -> Result<MigrationStep> {
    let alter = change_table(state, "people")?;
    let m = alter.add_column("aaddress_id", Type::Int);
    Ok(MigrationStep::new("add_address_to_people", m))
}

// Let add a column to people to wire the two together
fn rename_and_make_nullable(state: &TableState) -> Result<MigrationStep> {
    // fix the bad spelling :)
    let alter = change_table(state, "people")?;
    let m = alter.change("aaddress_id").null().rename("address_id");
    Ok(MigrationStep::new("rename_and_make_nullable", m))
}
