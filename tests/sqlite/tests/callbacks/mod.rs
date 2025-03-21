use welds::WeldsModel;
use welds::errors::Result;
use std::sync::Mutex;
use crate::get_conn;

#[derive(WeldsModel)]
#[welds(table = "Teams")]
#[welds(BeforeCreate(before_create))]
#[welds(BeforeCreate(before_create_async, async = true))]
#[welds(AfterCreate(after_create))]
#[welds(AfterCreate(after_create_async, async = true))]
#[welds(BeforeUpdate(before_update))]
#[welds(BeforeUpdate(before_update_async, async = true))]
#[welds(AfterUpdate(after_update))]
#[welds(AfterUpdate(after_update_async, async = true))]
#[welds(BeforeDelete(before_delete))]
#[welds(BeforeDelete(before_delete_async, async = true))]
#[welds(AfterDelete(after_delete))]
#[welds(AfterDelete(after_delete_async, async = true))]
pub struct Team {
    #[welds(primary_key)]
    pub id: i32,
    pub name: String,
    pub city_id: i32,
}

static CALLED: Mutex<Vec<String>> = Mutex::new(Vec::new());

fn record(callback: &str) {
    CALLED.lock().unwrap().push(callback.to_string())
}

async fn record_async(callback: &str) {
    CALLED.lock().unwrap().push(callback.to_string())
}

fn clear_called() {
    *CALLED.lock().unwrap() = Vec::default()
}

fn before_create(team: &Team) -> Result<()> {
    record("before_create");
    Ok(())
}

async fn before_create_async(team: &Team) -> Result<()> {
    record_async("before_create_async").await;
    Ok(())
}

fn after_create(team: &Team) {
    record("after_create");
}

async fn after_create_async(team: &Team) {
    record_async("after_create_async").await;
}

fn before_update(team: &Team) -> Result<()> {
    record("before_update");
    Ok(())
}

async fn before_update_async(team: &Team) -> Result<()> {
    record_async("before_update_async").await;
    Ok(())
}

fn after_update(team: &Team) {
    record("after_update");
}

async fn after_update_async(team: &Team) {
    record_async("after_update_async").await;
}

fn before_delete(team: &Team) -> Result<()> {
    record("before_delete");
    Ok(())
}

async fn before_delete_async(team: &Team) -> Result<()> {
    record_async("before_delete_async").await;
    Ok(())
}

fn after_delete(team: &Team) {
    record("after_delete");
}

async fn after_delete_async(team: &Team) {
    record_async("after_delete_async").await;
}

#[test]
fn should_handle_all_callbacks() {
    async_std::task::block_on(async {
        let conn = get_conn().await;

        // Create
        let mut team = Team::new();
        team.name = "Created".to_string();
        team.city_id = 1;
        team.save(&conn).await.unwrap();

        assert_eq!(
            CALLED.lock().unwrap().as_slice(),
            ["before_create", "before_create_async", "after_create", "after_create_async"]
        );

        clear_called();

        // Update
        team.name = "Updated".to_string();
        team.save(&conn).await.unwrap();

        assert_eq!(
            CALLED.lock().unwrap().as_slice(),
            ["before_update", "before_update_async", "after_update", "after_update_async"]
        );

        clear_called();

        // Delete
        team.delete(&conn).await.unwrap();

        assert_eq!(
            CALLED.lock().unwrap().as_slice(),
            ["before_delete", "before_delete_async", "after_delete", "after_delete_async"]
        );

        clear_called();
    });
}
