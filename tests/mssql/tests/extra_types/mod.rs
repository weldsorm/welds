use super::get_conn;
use uuid::Uuid;
use welds::prelude::*;

#[derive(WeldsModel, Clone, PartialEq, Debug)]
#[welds(schema = "welds", table = "extra_types")]
pub struct ExtraType {
    #[welds(primary_key)]
    pub id: uuid::Uuid,
    pub date_col: chrono::NaiveDate,
    pub time_col: chrono::NaiveTime,
    pub datetime_col: chrono::NaiveDateTime,
    pub datetimetz_col: chrono::DateTime<chrono::FixedOffset>,
}

#[tokio::test]
async fn should_be_able_to_save_load_extra_types() {
    let conn = get_conn().await;
    let trans = conn.begin().await.unwrap();

    let org = ExtraType {
        id: Uuid::new_v4(),
        date_col: chrono::Utc::now().date_naive(),
        time_col: chrono::Utc::now().naive_local().time(),
        datetime_col: chrono::Utc::now().naive_local(),
        datetimetz_col: chrono::Utc::now().into(),
    };

    let mut start = DbState::new_uncreated(org.clone());

    start.save(&trans).await.unwrap();

    let loaded = ExtraType::find_by_id(&trans, org.id)
        .await
        .unwrap()
        .unwrap();

    // make sure the object hasn't changed.
    // the DB might truncate the nano seconds.
    assert_eq!(loaded.as_ref().id, org.id);
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

    trans.rollback().await.unwrap();
}
