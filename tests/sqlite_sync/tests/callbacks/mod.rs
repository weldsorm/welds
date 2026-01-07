use crate::get_conn;
use std::sync::Mutex;
use welds::errors::Result;
use welds::WeldsModel;

#[derive(WeldsModel)]
#[welds(table = "Teams")]
#[welds(BeforeCreate(before_create))]
#[welds(AfterCreate(after_create))]
#[welds(BeforeUpdate(before_update))]
#[welds(AfterUpdate(after_update))]
#[welds(BeforeDelete(before_delete))]
#[welds(AfterDelete(after_delete))]
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

fn clear_called() {
    *CALLED.lock().unwrap() = Vec::default()
}

fn before_create(team: &Team) -> Result<()> {
    record("before_create");
    Ok(())
}

fn after_create(team: &Team) {
    record("after_create");
}

fn before_update(team: &Team) -> Result<()> {
    record("before_update");
    Ok(())
}

fn after_update(team: &Team) {
    record("after_update");
}

fn before_delete(team: &Team) -> Result<()> {
    record("before_delete");
    Ok(())
}

fn after_delete(team: &Team) {
    record("after_delete");
}

#[test]
fn should_handle_all_callbacks() {
    let conn = get_conn();

    // Create
    let mut team = Team::new();
    team.name = "Created".to_string();
    team.city_id = 1;
    team.save(&conn).unwrap();

    assert_eq!(
        CALLED.lock().unwrap().as_slice(),
        [
            "before_create",
            "before_create_async",
            "after_create",
            "after_create_async"
        ]
    );

    clear_called();

    // Update
    team.name = "Updated".to_string();
    team.save(&conn).unwrap();

    assert_eq!(
        CALLED.lock().unwrap().as_slice(),
        [
            "before_update",
            "before_update_async",
            "after_update",
            "after_update_async"
        ]
    );

    clear_called();

    // Delete
    team.delete(&conn).unwrap();

    assert_eq!(
        CALLED.lock().unwrap().as_slice(),
        [
            "before_delete",
            "before_delete_async",
            "after_delete",
            "after_delete_async"
        ]
    );

    clear_called();
}
