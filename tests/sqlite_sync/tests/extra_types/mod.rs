use super::get_conn;
use serde_json::json;
use uuid::Uuid;
use welds::prelude::*;

#[derive(WeldsModel, Clone, PartialEq, Debug)]
#[welds(table = "extra_types")]
pub struct ExtraType {
    #[welds(primary_key)]
    pub id: uuid::Uuid,
    // left in here so sql matches async version of DB
    pub json_col: String,
    pub date_col: chrono::NaiveDate,
    pub time_col: chrono::NaiveTime,
    pub datetime_col: chrono::NaiveDateTime,
    pub datetimetz_col: chrono::DateTime<chrono::Utc>,
}

#[test]
fn should_be_able_to_save_load_extra_types() {
    let conn = get_conn();
    let trans = conn.begin().unwrap();

    let org = ExtraType {
        id: Uuid::new_v4(),
        json_col: json!({"woot": true}).to_string(),
        date_col: chrono::Utc::now().date_naive(),
        time_col: chrono::Utc::now().naive_local().time(),
        datetime_col: chrono::Utc::now().naive_local(),
        datetimetz_col: chrono::Utc::now(),
    };

    let mut start = DbState::new_uncreated(org.clone());

    start.save(&trans).unwrap();

    let loaded = ExtraType::find_by_id(&trans, org.id).unwrap().unwrap();

    // make sure the object hasn't changed.
    // the DB might truncate the nano seconds.
    assert_eq!(loaded.as_ref().id, org.id);
    assert_eq!(&loaded.as_ref().json_col, &org.json_col);
    assert_eq!(loaded.as_ref().date_col, org.date_col);

    let diff = loaded.as_ref().time_col.signed_duration_since(org.time_col);
    assert!(diff.num_milliseconds() < 1);

    let diff = loaded
        .as_ref()
        .datetime_col
        .signed_duration_since(org.datetime_col);
    assert!(diff.num_milliseconds() < 1);

    let diff = loaded
        .as_ref()
        .datetimetz_col
        .signed_duration_since(org.datetimetz_col);
    assert!(diff.num_milliseconds() < 1);

    trans.rollback().unwrap();
}
