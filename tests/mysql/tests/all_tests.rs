use mysql_test::models::product::{BadProductColumns, BadProductMissingTable, Product};
use sqlx::MySql;
use welds::connection::Pool;

#[derive(Default, Debug, Clone, sqlx::FromRow)]
pub struct Count {
    pub count: i32,
}

#[test]
fn should_be_able_to_connect() {
    async_std::task::block_on(async {
        let sqlx_conn = testlib::mysql::conn().await.unwrap();
        let conn: Pool<MySql> = sqlx_conn.into();
        assert!(!conn.as_sqlx_pool().is_closed());
    })
}

#[test]
fn should_be_able_to_read_all_products() {
    async_std::task::block_on(async {
        let sqlx_conn = testlib::mysql::conn().await.unwrap();
        let conn: Pool<MySql> = sqlx_conn.into();
        let q = Product::all();
        eprintln!("SQL: {}", q.to_sql());
        let all = q.run(&conn).await.unwrap();
        assert_eq!(all.len(), 6, "Unexpected number of rows returned");
    })
}

#[test]
fn should_be_able_to_filter_on_id() {
    async_std::task::block_on(async {
        let sqlx_conn = testlib::mysql::conn().await.unwrap();
        let conn: Pool<MySql> = sqlx_conn.into();
        let q = Product::where_col(|x| x.id.equal(1));
        eprintln!("SQL: {}", q.to_sql());
        let just_horse = q.run(&conn).await.unwrap();
        assert_eq!(
            just_horse.len(),
            1,
            "Expected to only find the horse in the test data"
        );
    })
}

#[test]
fn should_be_able_to_filter_on_equal() {
    async_std::task::block_on(async {
        let sqlx_conn = testlib::mysql::conn().await.unwrap();
        let conn: Pool<MySql> = sqlx_conn.into();
        let q = Product::where_col(|x| x.price1.equal(1.10));
        eprintln!("SQL: {}", q.to_sql());
        let just_horse = q.run(&conn).await.unwrap();
        assert_eq!(
            just_horse.len(),
            1,
            "Expected to only find the horse in the test data"
        );
    })
}

#[test]
fn should_be_able_to_filter_on_lt() {
    async_std::task::block_on(async {
        let sqlx_conn = testlib::mysql::conn().await.unwrap();
        let conn: Pool<MySql> = sqlx_conn.into();
        let q = Product::where_col(|x| x.price1.lt(3.00));
        eprintln!("SQL: {}", q.to_sql());
        let data = q.run(&conn).await.unwrap();
        assert_eq!(data.len(), 2, "Expected horse and dog",);
    })
}

#[test]
fn should_be_able_to_filter_on_lte() {
    async_std::task::block_on(async {
        let sqlx_conn = testlib::mysql::conn().await.unwrap();
        let conn: Pool<MySql> = sqlx_conn.into();
        let q = Product::where_col(|x| x.price1.lte(2.10));
        eprintln!("SQL: {}", q.to_sql());
        let data = q.run(&conn).await.unwrap();
        assert_eq!(data.len(), 2, "Expected horse and dog",);
    })
}

#[test]
fn should_be_able_to_filter_with_nulls() {
    async_std::task::block_on(async {
        let sqlx_conn = testlib::mysql::conn().await.unwrap();
        let conn: Pool<MySql> = sqlx_conn.into();
        // is null
        let q1 = Product::where_col(|x| x.price1.equal(None));
        eprintln!("SQL_1: {}", q1.to_sql());
        let data1 = q1.run(&conn).await.unwrap();
        assert_eq!(data1.len(), 0, "Expected All",);
        // is not null
        let q1 = Product::where_col(|x| x.price1.not_equal(None));
        eprintln!("SQL_2: {}", q1.to_sql());
        let data1 = q1.run(&conn).await.unwrap();
        assert_eq!(data1.len(), 6, "Expected All",);
    })
}

#[test]
fn should_be_able_to_count_in_sql() {
    async_std::task::block_on(async {
        let sqlx_conn = testlib::mysql::conn().await.unwrap();
        let conn: Pool<MySql> = sqlx_conn.into();
        let q = Product::where_col(|x| x.price1.lte(2.10));
        eprintln!("SQL: {}", q.to_sql());
        let count = q.count(&conn).await.unwrap();
        assert_eq!(count, 2,);
    })
}

#[test]
fn should_be_able_to_limit_results_in_sql() {
    async_std::task::block_on(async {
        let sqlx_conn = testlib::mysql::conn().await.unwrap();
        let conn: Pool<MySql> = sqlx_conn.into();
        let q = Product::all().limit(2).offset(1);
        eprintln!("SQL: {}", q.to_sql());
        let count = q.run(&conn).await.unwrap().len();
        assert_eq!(count, 2);
    })
}

#[test]
fn should_be_able_to_order_by_id() {
    async_std::task::block_on(async {
        let sqlx_conn = testlib::mysql::conn().await.unwrap();
        let conn: Pool<MySql> = sqlx_conn.into();
        let q = Product::all().order_by_asc(|x| x.id);
        eprintln!("SQL: {}", q.to_sql());
        let all = q.run(&conn).await.unwrap();
        let ids: Vec<i32> = all.iter().map(|x| x.id).collect();
        let mut ids_sorted = ids.clone();
        ids_sorted.sort();
        assert_eq!(ids, ids_sorted);
    })
}

#[test]
fn should_be_able_to_update_a_product() {
    async_std::task::block_on(async {
        let sqlx_conn = testlib::mysql::conn().await.unwrap();
        let conn: Pool<MySql> = sqlx_conn.into();
        let mut trans = conn.begin().await.unwrap();

        let q = Product::all().limit(1);
        let mut found: Vec<_> = q.run(&mut trans).await.unwrap();
        let mut p1 = found.pop().unwrap();
        p1.name = "Test1".to_owned();
        p1.save(&mut trans).await.unwrap();

        let q = Product::where_col(|x| x.id.equal(p1.id));
        let mut found: Vec<_> = q.run(&mut trans).await.unwrap();
        let p2 = found.pop().unwrap();
        assert_eq!(p2.name, "Test1");

        trans.rollback().await.unwrap();
    })
}

#[test]
fn should_be_able_to_create_a_new_product() {
    async_std::task::block_on(async {
        let sqlx_conn = testlib::mysql::conn().await.unwrap();
        let conn: Pool<MySql> = sqlx_conn.into();
        let mut trans = conn.begin().await.unwrap();

        let mut p1 = Product::new();
        p1.name = "newyNewFace".to_owned();
        p1.description = Some("YES!".to_owned());
        // Note: creation will set the PK for the model.
        p1.save(&mut trans).await.unwrap();

        let q = Product::where_col(|x| x.id.equal(p1.id));
        let mut found: Vec<_> = q.run(&mut trans).await.unwrap();
        let p2 = found.pop().unwrap();
        assert_eq!(p2.name, "newyNewFace");

        trans.rollback().await.unwrap();
    })
}

#[test]
fn should_be_able_to_scan_for_all_tables() {
    async_std::task::block_on(async {
        let sqlx_conn = testlib::mysql::conn().await.unwrap();
        let conn: Pool<MySql> = sqlx_conn.into();
        let tables = welds::detect::find_tables(&conn).await.unwrap();
        assert_eq!(2, tables.len());
    })
}

#[test]
fn a_model_should_be_able_to_verify_its_schema_missing_table() {
    async_std::task::block_on(async {
        let sqlx_conn = testlib::mysql::conn().await.unwrap();
        let conn: Pool<MySql> = sqlx_conn.into();
        let issues = welds::check::schema::<BadProductMissingTable, _, _>(&conn)
            .await
            .unwrap();
        assert_eq!(issues.len(), 1);
        let issue = &issues[0];
        assert_eq!(issue.kind, welds::check::Kind::MissingTable);
    })
}

#[test]
fn a_model_should_be_able_to_verify_its_schema_missing_column() {
    async_std::task::block_on(async {
        let sqlx_conn = testlib::mysql::conn().await.unwrap();
        let conn: Pool<MySql> = sqlx_conn.into();
        let issues = welds::check::schema::<BadProductColumns, _, _>(&conn)
            .await
            .unwrap();
        // NOTE: a column name changed so it is added on the model and removed in the db giving two warnings
        for issue in &issues {
            eprintln!("{}", issue);
        }
        assert_eq!(issues.len(), 7);
    })
}
