use super::get_conn;
use serde_json::json;
use uuid::Uuid;
use welds::prelude::*;

#[derive(WeldsModel, Clone, PartialEq, Debug)]
#[welds(table = "extra_types")]
pub struct ExtraType {
    #[welds(primary_key)]
    pub id: String,
    pub json_col: serde_json::Value,
    pub date_col: chrono::NaiveDate,
    pub time_col: chrono::NaiveTime,
    pub datetime_col: chrono::NaiveDateTime,
    pub datetimetz_col: chrono::DateTime<chrono::Utc>,
}

#[test]
fn should_be_able_to_save_load_extra_types() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let trans = conn.begin().await.unwrap();

        let org = ExtraType {
            id: Uuid::new_v4().to_string(),
            json_col: json!({"woot": true}),
            date_col: chrono::Utc::now().date_naive(),
            time_col: chrono::Utc::now().naive_local().time(),
            datetime_col: chrono::Utc::now().naive_local(),
            datetimetz_col: chrono::Utc::now(),
        };

        let mut start = DbState::new_uncreated(org.clone());

        start.save(&trans).await.unwrap();

        let loaded = ExtraType::find_by_id(&trans, org.id.as_str())
            .await
            .unwrap()
            .unwrap();

        // make sure the object hasn't changed.
        // the DB might truncate the nano seconds.
        assert_eq!(loaded.as_ref().id.as_str(), org.id.as_str());
        assert_eq!(&loaded.as_ref().json_col, &org.json_col);
        assert_eq!(loaded.as_ref().date_col, org.date_col);

        let diff = loaded.as_ref().time_col.signed_duration_since(org.time_col);
        assert!(diff.num_seconds() < 1);

        let diff = loaded
            .as_ref()
            .datetime_col
            .signed_duration_since(org.datetime_col);
        assert!(diff.num_seconds() < 1);

        let diff = loaded
            .as_ref()
            .datetimetz_col
            .signed_duration_since(org.datetimetz_col);
        assert!(diff.num_seconds() < 1);

        trans.rollback().await.unwrap();
    })
}

#[test]
fn should_be_able_to_lg_gt_dates() {
    async_std::task::block_on(async {
        let now_tz = chrono::Local::now();
        let now_native = chrono::Local::now().naive_local();
        let now_time = now_tz.time();
        let now_date = now_tz.date_naive();

        // NOTE: the test here is that all these compile
        let _q1 = ExtraType::all().where_col(|x| x.datetimetz_col.lt(now_tz));
        let _q2 = ExtraType::all().where_col(|x| x.datetime_col.lt(now_native));
        let _q3 = ExtraType::all().where_col(|x| x.time_col.lt(now_time));
        let _q4 = ExtraType::all().where_col(|x| x.date_col.lt(now_date));

        let _q1 = ExtraType::all().where_col(|x| x.datetimetz_col.lte(now_tz));
        let _q2 = ExtraType::all().where_col(|x| x.datetime_col.lte(now_native));
        let _q3 = ExtraType::all().where_col(|x| x.time_col.lte(now_time));
        let _q4 = ExtraType::all().where_col(|x| x.date_col.lte(now_date));

        let _q1 = ExtraType::all().where_col(|x| x.datetimetz_col.gt(now_tz));
        let _q2 = ExtraType::all().where_col(|x| x.datetime_col.gt(now_native));
        let _q3 = ExtraType::all().where_col(|x| x.time_col.gt(now_time));
        let _q4 = ExtraType::all().where_col(|x| x.date_col.gt(now_date));

        let _q1 = ExtraType::all().where_col(|x| x.datetimetz_col.gte(now_tz));
        let _q2 = ExtraType::all().where_col(|x| x.datetime_col.gte(now_native));
        let _q3 = ExtraType::all().where_col(|x| x.time_col.gte(now_time));
        let _q4 = ExtraType::all().where_col(|x| x.date_col.gte(now_date));
    })
}
