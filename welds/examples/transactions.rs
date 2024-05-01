use welds::errors::Result;
use welds::errors::WeldsError;
use welds::migrations::{create_table, types::Type, up, MigrationStep, TableState};
use welds::prelude::*;
use welds_connections::Transaction;

// just a simple migration to setup the peoples table. not needed for this example
fn db_setup(_: &TableState) -> Result<MigrationStep> {
    let m = create_table("people")
        .id(|c| c("id", Type::Int))
        .column(|c| c("name", Type::String));
    Ok(MigrationStep::new("db_setup", m))
}

#[derive(WeldsModel, Debug)]
#[welds(table = "people")]
struct Person {
    #[welds(primary_key)]
    id: i32,
    name: String,
}

#[async_std::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    // Connect and setup a DB for use to play with
    let client = welds::connections::sqlite::connect("sqlite::memory:").await?;
    up(&client, &[db_setup]).await?;

    // run some SQL with an error. it will rollback the transaction
    let _ = create_with_errors(&client).await;
    let count = Person::all().count(&client).await?;
    assert_eq!(count, 0);
    println!("PEOPLE COUNT: {}", 0);

    // run some SQL and commit. it will commit the transaction
    let _ = create_person(&client).await;
    let count = Person::all().count(&client).await?;
    assert_eq!(count, 1);
    println!("PEOPLE COUNT: {}", 1);

    Ok(())
}

/// Create a person, but errors out before committing
async fn create_with_errors(client: &(dyn TransactStart)) -> Result<()> {
    //start the transaction
    let transaction = client.begin().await?;

    // extra layer of calling/passing transaction to show how
    create_person_inner_alt(&transaction).await?;

    // return some random error.
    // no commit was ever called, and transaction has gone out of scope
    // This will rollback the transaction
    Err(WeldsError::RowNowFound)?
}

/// Create a person, commits it
async fn create_person(client: &(dyn TransactStart)) -> Result<()> {
    //start the transaction
    let transaction = client.begin().await?;

    // extra layer of calling/passing transaction to show how
    create_person_inner(&transaction).await?;

    transaction.commit().await?;
    Ok(())
}

// NOTE: no need to force other developers to use a transaction, all transactions are clients
async fn create_person_inner(transaction: &dyn Client) -> Result<()> {
    //save the new person
    let mut p = DbState::new_uncreated(Person {
        id: 0,
        name: "Bobby".to_owned(),
    });
    p.save(transaction).await?;
    Ok(())
}

// If you want to force logic to occur in a transactional state, you can use Transaction
async fn create_person_inner_alt<'t>(transaction: &Transaction<'t>) -> Result<()> {
    //save the new person
    let mut p = DbState::new_uncreated(Person {
        id: 0,
        name: "Bobby".to_owned(),
    });
    p.save(transaction).await?;
    Ok(())
}
