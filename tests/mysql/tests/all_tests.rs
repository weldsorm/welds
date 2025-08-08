use mysql_test::models::order::Order;
use mysql_test::models::product::{BadProductColumns, BadProductMissingTable, Product};
use mysql_test::models::StringThing;
use mysql_test::models::Thing1;
use std::env;
use welds::connections::mysql::MysqlClient;
use welds::state::{DbState, DbStatus};
use welds::Syntax;
use welds::TransactStart;

mod extra_types;
mod group_by;
mod migrations;
mod streams;

async fn get_conn() -> MysqlClient {
    // Allow the tester to control the database to test against.
    // WARNING: if you take control of the database connection, YOU are resonsible
    // for making it a valid test database. checkout: testlib/database/mysql
    match env::var("TEST_DATABASE_URL") {
        Ok(cs) => welds::connections::mysql::connect(&cs).await.unwrap(),
        Err(_) => {
            let conn = testlib::mysql::conn().await.unwrap();
            let client: MysqlClient = conn.into();
            client
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Count {
    pub count: i32,
}

#[test]
fn should_be_able_to_connect() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        assert!(!conn.as_sqlx_pool().is_closed());
    })
}

#[test]
fn should_be_able_to_read_all_products() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let q = Product::all();
        eprintln!("SQL: {}", q.to_sql(Syntax::Mysql));
        let all = q.run(&conn).await.unwrap();
        assert_eq!(all.len(), 6, "Unexpected number of rows returned");
    })
}

#[test]
fn should_be_able_to_filter_on_id() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let q = Product::where_col(|x| x.id.equal(1));
        eprintln!("SQL: {}", q.to_sql(Syntax::Mysql));
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
        let conn = get_conn().await;
        let q = Product::where_col(|x| x.price1.equal(1.10));
        eprintln!("SQL: {}", q.to_sql(Syntax::Mysql));
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
        let conn = get_conn().await;
        let q = Product::where_col(|x| x.price1.lt(3.00));
        eprintln!("SQL: {}", q.to_sql(Syntax::Mysql));
        let data = q.run(&conn).await.unwrap();
        assert_eq!(data.len(), 2, "Expected horse and dog",);
    })
}

#[test]
fn should_be_able_to_filter_on_lte() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let q = Product::where_col(|x| x.price1.lte(2.10));
        eprintln!("SQL: {}", q.to_sql(Syntax::Mysql));
        let data = q.run(&conn).await.unwrap();
        assert_eq!(data.len(), 2, "Expected horse and dog",);
    })
}

#[test]
fn should_be_able_to_filter_with_nulls() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        // is null
        let q1 = Product::where_col(|x| x.price1.equal(None));
        eprintln!("SQL_1: {}", q1.to_sql(Syntax::Mysql));
        let data1 = q1.run(&conn).await.unwrap();
        assert_eq!(data1.len(), 0, "Expected All",);
        // is not null
        let q1 = Product::where_col(|x| x.price1.not_equal(None));
        eprintln!("SQL_2: {}", q1.to_sql(Syntax::Mysql));
        let data1 = q1.run(&conn).await.unwrap();
        assert_eq!(data1.len(), 6, "Expected All",);
    })
}

#[test]
fn should_be_able_to_count_in_sql() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let q = Product::where_col(|x| x.price1.lte(2.10));
        eprintln!("SQL: {}", q.to_sql(Syntax::Mysql));
        let count = q.count(&conn).await.unwrap();
        assert_eq!(count, 2,);
    })
}

#[test]
fn should_be_able_to_limit_results_in_sql() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let q = Product::all().limit(2).offset(1);
        eprintln!("SQL: {}", q.to_sql(Syntax::Mysql));
        let count = q.run(&conn).await.unwrap().len();
        assert_eq!(count, 2);
    })
}

#[test]
fn should_be_able_to_order_by_id() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let q = Product::all().order_by_asc(|x| x.id);
        eprintln!("SQL: {}", q.to_sql(Syntax::Mysql));
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
        let conn = get_conn().await;
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
        let conn = get_conn().await;
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
        let conn = get_conn().await;
        let tables = welds::detect::find_tables(&conn).await.unwrap();
        assert!(tables.len() >= 12);
    })
}

#[test]
fn a_model_should_be_able_to_verify_its_schema_missing_table() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let issues = welds::check::schema::<BadProductMissingTable>(&conn)
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
        let conn = get_conn().await;
        let issues = welds::check::schema::<BadProductColumns>(&conn)
            .await
            .unwrap();
        // NOTE: a column name changed so it is added on the model and removed in the db giving two warnings
        for issue in &issues {
            eprintln!("{}", issue);
        }
        assert_eq!(issues.len(), 7);
    })
}

#[test]
fn should_be_able_to_bulk_delete() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let trans = conn.begin().await.unwrap();
        let p1 = Product::all()
            .limit(1)
            .run(&trans)
            .await
            .unwrap()
            .pop()
            .unwrap();
        let mut order = Order::new();
        order.product_id = p1.id;
        order.save(&trans).await.unwrap();
        let q = Order::all().where_col(|x| x.id.gt(0));
        let count = q.count(&trans).await.unwrap();
        q.delete(&trans).await.unwrap();
        assert!(count > 0);
        trans.rollback().await.unwrap();
    })
}

#[test]
fn should_be_able_to_bulk_update() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let q = Order::all()
            .where_col(|x| x.code.equal(None))
            .set(|x| x.code, "test");
        let sql = q.to_sql(Syntax::Mysql);
        eprintln!("SQL: {}", sql);
        q.run(&conn).await.unwrap();
    })
}

#[test]
fn should_be_able_to_bulk_update2() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let q = Product::all()
            .map_query(|p| p.orders)
            .set(|x| x.code, "test2");
        let sql = q.to_sql(Syntax::Mysql);
        eprintln!("SQL: {}", sql);
        q.run(&conn).await.unwrap();
    })
}

#[test]
fn should_be_able_to_bulk_update_by_set_col() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let q = Product::all()
            .map_query(|p| p.orders)
            .where_col(|c| c.id.equal(2342534))
            .set_col(|x| x.code.equal("test2"));
        let sql = q.to_sql(Syntax::Postgres);
        eprintln!("SQL: {}", sql);
        q.run(&conn).await.unwrap();
    })
}

#[test]
fn should_be_able_to_bulk_insert() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let trans = conn.begin().await.unwrap();
        let things: Vec<_> = (0..3000)
            .map(|x| Thing1 {
                id: 0,
                value: format!("Bulk_Insert: {}", x),
            })
            .collect();
        welds::query::insert::bulk_insert(&trans, &things)
            .await
            .unwrap();
        trans.rollback().await.unwrap();
    })
}

#[test]
fn should_be_able_to_create_a_model_with_a_string_id() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let trans = conn.begin().await.unwrap();
        let mut thing = DbState::new_uncreated(StringThing {
            id: "test".to_owned(),
            value: "test".to_owned(),
        });
        thing.save(&trans).await.unwrap();
        assert_eq!(thing.db_status(), DbStatus::NotModified);
        let found = StringThing::find_by_id(&trans, "test".to_owned())
            .await
            .unwrap();
        assert!(found.is_some());
        trans.rollback().await.unwrap();
    })
}

#[test]
fn should_be_able_to_write_custom_wheres() {
    async_std::task::block_on(async {
        use welds::query::builder::ManualParam;
        let conn = get_conn().await;

        // find a known in DB row
        let mut knowns = Product::all().limit(1).run(&conn).await.unwrap();
        let known = knowns.pop().unwrap();
        let known_id = known.id;

        let params = ManualParam::new().push(known_id);
        // run the custom where
        let found = Product::all()
            .where_manual(|c| c.id, "=?", params)
            .run(&conn)
            .await
            .unwrap()
            .pop()
            .unwrap();
        assert_eq!(found.id, known_id);
    })
}

#[test]
fn should_be_able_to_write_a_custom_set() {
    async_std::task::block_on(async {
        use welds::query::builder::ManualParam;
        let params = ManualParam::new().push(1);
        let conn = get_conn().await;
        let q = Product::all()
            .map_query(|p| p.orders)
            .where_col(|c| c.id.equal(2342534))
            .set_manual(|x| x.product_id, "product_id + ?", params);
        let sql = q.to_sql(Syntax::Postgres);
        eprintln!("SQL: {}", sql);
        q.run(&conn).await.unwrap();
    })
}
