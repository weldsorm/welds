use super::get_conn;
use welds::detect::find_table;
use welds::errors::Result;
use welds::migrations::types::Type;
use welds::migrations::MigrationFn;
use welds::migrations::MigrationStep;
use welds::migrations::{change_table, create_table, TableState};
use welds::migrations::{down, up};
use welds::Client;

/************************************************
* two migrations shouldn't have the same name
* **********************************************/

fn bad_migration(_state: &TableState) -> Result<MigrationStep> {
    let m = create_table("asdlkfj230iuasodjfhor2").id(|c| c("id", Type::Int));
    Ok(MigrationStep::new("Bad Migration Name", m))
}

#[tokio::test]
async fn two_migrations_with_the_same_name_should_be_an_error() {
    let client = get_conn().await;
    let client = &client;
    let list: Vec<MigrationFn> = vec![bad_migration, bad_migration];
    let result = up(client, list.as_slice()).await;
    assert!(result.is_err())
}

/************************************************
 * Test creating a table and rolling that back
 * **********************************************/

fn test_create_table_migration(_state: &TableState) -> Result<MigrationStep> {
    let m = create_table("blarf")
        .id(|c| c("id", Type::Int))
        .column(|c| c("name", Type::String));
    Ok(MigrationStep::new("Create Blarf Table", m))
}

#[tokio::test]
async fn should_be_able_to_create_a_table() {
    //env_logger::init();
    let client = get_conn().await;
    let client = &client;

    // make sure the table doesn't exist
    let table = find_table(None as Option<&str>, "blarf", client)
        .await
        .unwrap();
    assert!(table.is_none());

    // Run the migration
    let list: Vec<MigrationFn> = vec![test_create_table_migration];
    up(client, list.as_slice()).await.unwrap();

    // make sure the table exists
    let table = find_table(None as Option<&str>, "blarf", client)
        .await
        .unwrap();
    assert!(table.is_some(), "expected table to exist");

    // down the migration
    down(client, "Create Blarf Table").await.unwrap();

    // make sure the table doesn't exist
    let table = find_table(None as Option<&str>, "blarf", client)
        .await
        .unwrap();
    assert!(table.is_none());
}

/************************************************
 * Test dropping a table and rolling that back
 * **********************************************/

fn drop_table_setup_1(_state: &TableState) -> Result<MigrationStep> {
    let m = create_table("asdfasdfasdfasdf1")
        .id(|c| c("id", Type::Int))
        .column(|c| c("da_numbers", Type::FloatBig));
    Ok(MigrationStep::new("droptabletest_m1", m))
}

fn drop_table_m2(state: &TableState) -> Result<MigrationStep> {
    let m = change_table(state, "asdfasdfasdfasdf1")?.drop();

    Ok(MigrationStep::new("droptabletest_m2", m))
}

#[tokio::test]
async fn should_be_able_to_drop_a_table() {
    let client = get_conn().await;
    let client = &client;
    let tablename = "asdfasdfasdfasdf1";

    // make sure the table we are testing the drop for exists
    let list1: Vec<MigrationFn> = vec![drop_table_setup_1];
    up(client, list1.as_slice()).await.unwrap();
    let table = find_table(None as Option<&str>, tablename, client)
        .await
        .unwrap();
    assert!(table.is_some());

    // test dropping the table
    let list1: Vec<MigrationFn> = vec![drop_table_m2];
    up(client, list1.as_slice()).await.unwrap();
    let table = find_table(None as Option<&str>, tablename, client)
        .await
        .unwrap();
    assert!(table.is_none());

    // down the drop migration should recreated it
    down(client, "droptabletest_m2").await.unwrap();

    // make sure the table doesn't exist
    let table = find_table(None as Option<&str>, tablename, client)
        .await
        .unwrap();
    assert!(table.is_some());
}

/************************************************
 * Test renaming a column and rolling that back
 * **********************************************/

fn rename_column_setup(_state: &TableState) -> Result<MigrationStep> {
    let m = create_table("tabletabletable2")
        .id(|c| c("id", Type::Int))
        .column(|c| c("da_numbers", Type::FloatBig));
    Ok(MigrationStep::new("test_rename_column_1", m))
}

fn rename_column_test_migration(state: &TableState) -> Result<MigrationStep> {
    let m = change_table(state, "tabletabletable2")?
        .change("da_numbers")
        .rename("da_numbers_new");
    Ok(MigrationStep::new("test_rename_column_2", m))
}

#[tokio::test]
async fn should_be_able_to_rename_a_column() {
    let client = get_conn().await;
    let client = &client;
    let tablename = "tabletabletable2";

    let list1: Vec<MigrationFn> = vec![rename_column_setup, rename_column_test_migration];
    up(client, list1.as_slice()).await.unwrap();
    let namespace: Option<&str> = None;
    let table = find_table(namespace, tablename, client).await.unwrap();
    let table = table.unwrap();
    let column = table
        .columns()
        .iter()
        .find(|c| c.name() == "da_numbers_new");
    assert!(column.is_some());

    // down the migration we are testing
    down(client, "test_rename_column_2").await.unwrap();

    // make sure the columns name was restored.
    let table = find_table(namespace, tablename, client).await.unwrap();
    let table = table.unwrap();
    let column = table.columns().iter().find(|c| c.name() == "da_numbers");
    assert!(column.is_some());
}

/************************************************
 * Test changing the type/null of a column
 * **********************************************/

fn change_type_setup(_state: &TableState) -> Result<MigrationStep> {
    let m = create_table("test_tabletabletable3")
        .id(|c| c("id", Type::Int))
        .column(|c| c("num", Type::Float).is_null());
    Ok(MigrationStep::new("test_change_type_1", m))
}

fn change_type_test_migration(state: &TableState) -> Result<MigrationStep> {
    let m = change_table(state, "test_tabletabletable3")?
        .change("num")
        .to_type(Type::Int)
        .not_null();
    Ok(MigrationStep::new("test_change_type_2", m))
}

#[tokio::test]
async fn should_be_able_to_change_a_type_without_dropping_data() {
    let client = get_conn().await;
    let client = &client;
    let tablename = "test_tabletabletable3";

    // make sure the table we are testing is all setup
    let list1: Vec<MigrationFn> = vec![change_type_setup];
    up(client, list1.as_slice()).await.unwrap();
    let add_data = format!("INSERT INTO {tablename} (num) VALUES (42.1)");
    client.execute(&add_data, &[]).await.unwrap();

    let list1: Vec<MigrationFn> = vec![change_type_setup, change_type_test_migration];
    up(client, list1.as_slice()).await.unwrap();

    // get info about the updated table to validate changes
    let namespace: Option<&str> = None;
    let table = find_table(namespace, tablename, client).await.unwrap();
    let table = table.unwrap();
    let column = table.columns().iter().find(|c| c.name() == "num");
    let column = column.unwrap();
    // check the table has changed
    assert_eq!(column.ty(), "INT");
    assert!(!column.null());

    // check the data is still there
    let count_col = "CAST( COUNT(*) as BIGINT )";
    let count_sql = format!("SELECT {count_col} AS BIGINT FROM {tablename}");
    let mut count_rows = client.fetch_rows(&count_sql, &[]).await.unwrap();
    let count_row = count_rows.pop().unwrap();
    let count: i64 = count_row.get_by_position(0).unwrap();
    assert!(count > 0);

    // down the migration restores the type
    down(client, "test_change_type_2").await.unwrap();

    // get info about the restored table to validate
    let namespace: Option<&str> = None;
    let table = find_table(namespace, tablename, client).await.unwrap();
    let table = table.unwrap();
    let column = table.columns().iter().find(|c| c.name() == "num");
    let column = column.unwrap();
    // check the table has changed
    assert_eq!(column.ty(), "REAL");
    assert!(column.null());
}

/************************************************
 * Test dropping the column lastname
 * **********************************************/

fn drop_column_test_setup(_state: &TableState) -> Result<MigrationStep> {
    let m = create_table("test_tabletabletable4")
        .id(|c| c("id", Type::Int))
        .column(|c| c("firstname", Type::String))
        .column(|c| c("lastname", Type::String));
    Ok(MigrationStep::new("test_drop_column_1", m))
}

fn drop_column_test_migration(state: &TableState) -> Result<MigrationStep> {
    let m = change_table(state, "test_tabletabletable4")?
        .change("lastname")
        .drop_column();
    Ok(MigrationStep::new("test_drop_column_2", m))
}

#[tokio::test]
async fn should_be_able_to_drop_a_column() {
    let client = get_conn().await;
    let client = &client;
    let tablename = "test_tabletabletable4";

    let list1: Vec<MigrationFn> = vec![drop_column_test_setup, drop_column_test_migration];
    up(client, list1.as_slice()).await.unwrap();

    // get info about the updated table to validate changes
    let namespace: Option<&str> = None;
    let table = find_table(namespace, tablename, client).await.unwrap();
    let table = table.unwrap();
    let column = table.columns().iter().find(|c| c.name() == "lastname");
    assert!(column.is_none(), "COL: {:?}", column);

    // down the migration restores the type
    down(client, "test_drop_column_2").await.unwrap();

    // make sure the column was restored
    let namespace: Option<&str> = None;
    let table = find_table(namespace, tablename, client).await.unwrap();
    let table = table.unwrap();
    let column = table.columns().iter().find(|c| c.name() == "lastname");
    assert!(column.is_some());
}

/************************************************
 * Test add the column lastname
 * **********************************************/

fn add_column_test_setup(_state: &TableState) -> Result<MigrationStep> {
    let m = create_table("test_tabletabletable5")
        .id(|c| c("id", Type::Int))
        .column(|c| c("firstname", Type::String));
    Ok(MigrationStep::new("test_add_column_1", m))
}

fn add_column_test_migration(state: &TableState) -> Result<MigrationStep> {
    let m = change_table(state, "test_tabletabletable5")?.add_column("lastname", Type::String);
    Ok(MigrationStep::new("test_add_column_2", m))
}

#[tokio::test]
async fn should_be_able_to_add_a_column() {
    let client = get_conn().await;
    let client = &client;
    let tablename = "test_tabletabletable5";

    let list1: Vec<MigrationFn> = vec![add_column_test_setup, add_column_test_migration];
    up(client, list1.as_slice()).await.unwrap();

    // get info about the updated table to validate changes
    let namespace: Option<&str> = None;
    let table = find_table(namespace, tablename, client).await.unwrap();
    let table = table.unwrap();
    let column = table.columns().iter().find(|c| c.name() == "lastname");
    assert!(column.is_some());

    // down the migration restores the type
    down(client, "test_add_column_2").await.unwrap();

    // make sure the column was restored
    let namespace: Option<&str> = None;
    let table = find_table(namespace, tablename, client).await.unwrap();
    let table = table.unwrap();
    let column = table.columns().iter().find(|c| c.name() == "lastname");
    assert!(column.is_none());
}

/// Make a manual Migration Step
///
fn manual_test_setup(_state: &TableState) -> Result<MigrationStep> {
    let m = create_table("test_table6")
        .id(|c| c("id", Type::Int))
        .column(|c| c("firstname", Type::String).is_null());
    Ok(MigrationStep::new("manual_test_1", m))
}

use welds::migrations::Manual;
fn manual_step(state: &TableState) -> Result<MigrationStep> {
    let m = Manual::up("update test_table6 set firstname = 'test' where firstname is null");
    Ok(MigrationStep::new("manual_test_2", m))
}

#[tokio::test]
async fn should_be_able_to_run_manual_migration_step() {
    let client = get_conn().await;
    let client = &client;

    let list1: Vec<MigrationFn> = vec![manual_test_setup, manual_step];
    up(client, list1.as_slice()).await.unwrap();
}
