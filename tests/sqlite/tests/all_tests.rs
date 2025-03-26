use sqlite_test::models::order::{Order, SmallOrder};
use sqlite_test::models::product::{BadProduct1, BadProduct2, Product};
use sqlite_test::models::StringThing;
use sqlite_test::models::{Thing1, Thing2, Thing3};
use welds::connections::sqlite::SqliteClient;
use welds::connections::TransactStart;
use welds::state::{DbState, DbStatus};
use welds::Syntax;

pub mod bulk_delete;
pub mod bulk_update;
pub mod callbacks;
pub mod extra_types;
pub mod includes;
pub mod migrations;
pub mod select_col;
pub mod sub_query_tests;
async fn get_conn() -> SqliteClient {
    let conn = testlib::sqlite::conn().await.unwrap();
    let client: SqliteClient = conn.into();
    client
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
        eprintln!("SQL: {}", q.to_sql(Syntax::Sqlite));
        let all = q.run(&conn).await.unwrap();
        assert_eq!(all.len(), 6, "Unexpected number of rows returned");
    })
}

#[test]
fn should_be_able_to_filter_on_id() {
    async_std::task::block_on(async {
        let conn = get_conn().await;

        let q = Product::where_col(|x| x.id.equal(1));
        eprintln!("SQL: {}", q.to_sql(Syntax::Sqlite));
        let just_horse = q.run(&conn).await.unwrap();
        assert_eq!(
            just_horse.len(),
            1,
            "Expected to only find the horse in the test data"
        );
    })
}

#[test]
fn should_lt() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let q = Product::where_col(|x| x.price1.lt(2.10));
        eprintln!("SQL: {}", q.to_sql(Syntax::Sqlite));
        let data = q.run(&conn).await.unwrap();
        assert_eq!(data.len(), 1);
    })
}

#[test]
fn should_be_able_to_filter_on_equal() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let q = Product::where_col(|x| x.id.equal(1));
        eprintln!("SQL: {}", q.to_sql(Syntax::Sqlite));
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
        eprintln!("SQL: {}", q.to_sql(Syntax::Sqlite));
        let data = q.run(&conn).await.unwrap();
        assert_eq!(data.len(), 2, "Expected horse and dog",);
    })
}

#[test]
fn should_be_able_to_filter_on_lte() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let q = Product::where_col(|x| x.id.lte(2));
        eprintln!("SQL: {}", q.to_sql(Syntax::Sqlite));
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
        eprintln!("SQL_1: {}", q1.to_sql(Syntax::Sqlite));
        let data1 = q1.run(&conn).await.unwrap();
        assert_eq!(data1.len(), 0, "Expected All",);
        // is not null
        let q1 = Product::where_col(|x| x.price1.not_equal(None));
        eprintln!("SQL_2: {}", q1.to_sql(Syntax::Sqlite));
        let data1 = q1.run(&conn).await.unwrap();
        assert_eq!(data1.len(), 6, "Expected All",);
    })
}

#[test]
fn should_be_able_to_count_in_sql() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let q = Product::where_col(|x| x.price1.lte(2.15));
        eprintln!("SQL: {}", q.to_sql_count(Syntax::Sqlite));
        let count = q.count(&conn).await.unwrap();
        assert_eq!(count, 2,);
    })
}

#[test]
fn should_be_able_to_limit_results_in_sql() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let q = Product::all().limit(2).offset(1);
        eprintln!("SQL: {}", q.to_sql(Syntax::Sqlite));
        let count = q.run(&conn).await.unwrap().len();
        assert_eq!(count, 2);
    })
}

#[test]
fn should_be_able_to_crud_with_small_int() {
    async_std::task::block_on(async {
        let client = get_conn().await;
        let trans = client.begin().await.unwrap();

        let mut p1 = Product::new();
        p1.save(&trans).await.unwrap();
        let mut p2 = Product::new();
        p2.save(&trans).await.unwrap();
        assert!(p1.id != 0);
        assert!(p2.id != 0);

        let mut s = SmallOrder::new();
        s.product_id = p1.id;
        s.save(&trans).await.unwrap();
        let s = SmallOrder::find_by_id(&trans, s.id).await.expect("db err");
        let mut s = s.expect("new SmallOrder not found");
        assert_eq!(s.product_id, p1.id);
        s.product_id = p2.id;
        s.save(&trans).await.unwrap();
        let s = SmallOrder::find_by_id(&trans, s.id).await.expect("db err");
        let mut s = s.expect("new SmallOrder not found");
        assert_eq!(s.product_id, p2.id);
        s.delete(&trans).await.expect("delete db err");
        let s_none = SmallOrder::find_by_id(&trans, s.id).await.expect("db err");
        assert!(s_none.is_none());

        trans.rollback().await.unwrap();
    })
}

#[test]
fn should_be_able_to_order_by_id() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let q = Product::all().order_by_asc(|x| x.id);
        eprintln!("SQL: {}", q.to_sql(Syntax::Sqlite));
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
        let trans = conn.begin().await.unwrap();

        let q = Product::all().limit(1);
        let mut found: Vec<_> = q.run(&trans).await.unwrap();
        let mut p1 = found.pop().unwrap();
        p1.name = "Test1".to_owned();
        p1.save(&trans).await.unwrap();

        let q = Product::where_col(|x| x.id.equal(p1.id));
        let mut found: Vec<_> = q.run(&trans).await.unwrap();
        let p2 = found.pop().unwrap();
        assert_eq!(p2.name, "Test1");

        trans.rollback().await.unwrap();
    })
}

#[test]
fn should_be_able_to_create_a_new_product() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let trans = conn.begin().await.unwrap();

        let mut p1 = Product::new();
        p1.name = "newyNewFace".to_owned();
        p1.description = Some("YES!".to_owned());
        // Note: creation will set the PK for the model.
        p1.save(&trans).await.unwrap();

        let q = Product::where_col(|x| x.id.equal(p1.id));
        let mut found: Vec<_> = q.run(&trans).await.unwrap();
        let p2 = found.pop().unwrap();
        assert_eq!(p2.name, "newyNewFace");
        assert!(p2.id != 0, "Expected new ID");

        let count = Product::where_col(|x| x.id.equal(p1.id))
            .count(&trans)
            .await
            .unwrap();
        assert_eq!(count, 1);

        trans.rollback().await.unwrap();
    })
}

#[test]
fn should_be_able_to_scan_for_all_tables() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let tables = welds::detect::find_all_tables(&conn).await.unwrap();
        assert_eq!(19, tables.len());
    })
}

#[test]
fn a_model_should_be_able_to_verify_its_schema_missing_table() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let issues = welds::check::schema::<BadProduct1>(&conn).await.unwrap();
        assert_eq!(issues.len(), 1);
        let issue = &issues[0];
        assert_eq!(issue.kind, welds::check::Kind::MissingTable);
    })
}

#[test]
fn a_model_should_be_able_to_verify_its_schema_missing_column() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let issues = welds::check::schema::<BadProduct2>(&conn).await.unwrap();
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
        let q = Product::all().map_query(|p| p.orders);
        let count = q.count(&trans).await.unwrap();
        q.delete(&trans).await.unwrap();
        assert!(count > 0);
        trans.rollback().await.unwrap();
    })
}

#[test]
fn should_be_able_to_bulk_delete2() {
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
        let sql = q.to_sql(Syntax::Sqlite);
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
        let sql = q.to_sql(Syntax::Sqlite);
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
fn should_be_able_to_limit_deletes() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let trans = conn.begin().await.unwrap();
        for _ in 0..100 {
            Thing1::new().save(&trans).await.unwrap();
        }
        let count_before = Thing1::all().count(&trans).await.unwrap();
        Thing1::all()
            .order_by_desc(|x| x.id)
            .limit(1)
            .delete(&trans)
            .await
            .unwrap();
        let count_after = Thing1::all().count(&trans).await.unwrap();
        assert_eq!(count_before - 1, count_after);
        trans.rollback().await.unwrap();
    })
}

#[test]
fn should_only_update_limited_rows_if_limit_is_in_query() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let trans = conn.begin().await.unwrap();
        for _ in 0..10 {
            Thing2::new().save(&trans).await.unwrap();
        }
        let update_statment = Thing2::all()
            .where_col(|x| x.id.gt(0))
            .order_by_desc(|x| x.id)
            .limit(1)
            .set(|x| x.value, "HAS_VALUE");

        let sql = update_statment.to_sql(Syntax::Sqlite);
        update_statment.run(&trans).await.unwrap();
        eprintln!("SQL: {}", sql);

        let count = Thing2::where_col(|x| x.value.equal("HAS_VALUE"))
            .count(&trans)
            .await
            .unwrap();
        assert_eq!(count, 1);
        trans.rollback().await.unwrap();
    })
}

#[test]
fn should_be_able_to_bulk_insert() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let trans = conn.begin().await.unwrap();
        let things: Vec<_> = (0..3000)
            .map(|x| Thing3 {
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
        let mut thing = DbState::new_uncreated(StringThing {
            id: "test".to_owned(),
            value: "test".to_owned(),
        });
        thing.save(&conn).await.unwrap();
        assert_eq!(thing.db_status(), DbStatus::NotModified);
        let found = StringThing::find_by_id(&conn, "test".to_owned())
            .await
            .unwrap();
        assert!(found.is_some());
    })
}

#[test]
fn should_be_able_to_set_a_nullable_value_to_null() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let trans = conn.begin().await.unwrap();

        Order::all()
            .where_col(|x| x.code.equal("333"))
            .set_null(|x| x.code)
            .run(&trans)
            .await
            .unwrap();

        trans.rollback().await.unwrap();
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
        let sql = q.to_sql(Syntax::Sqlite);
        eprintln!("SQL: {}", sql);
        q.run(&conn).await.unwrap();
    })
}

#[test]
fn should_be_able_to_write_a_custom_set3() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let q = Product::all()
            .map_query(|p| p.orders)
            .where_col(|c| c.id.equal(2342534))
            .set_manual(|x| x.product_id, "product_id + ? + ?", (42, 20.0));
        let sql = q.to_sql(Syntax::Sqlite);
        eprintln!("SQL: {}", sql);
        q.run(&conn).await.unwrap();
    })
}

#[test]
fn should_be_able_to_write_a_custom_set2() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let q = Product::all()
            .map_query(|p| p.orders)
            .where_col(|c| c.id.equal(2342534))
            .set_manual(|x| x.product_id, "product_id + ?", ());
        let sql = q.to_sql(Syntax::Sqlite);
        eprintln!("SQL: {}", sql);
        q.run(&conn).await.unwrap();
    })
}

#[test]
fn should_be_able_to_filter_by_multiple_values() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let query = Product::all().where_col(|p| p.id.in_list(&[2, 3, 4]));
        let results = query.run(&conn).await.unwrap();
        assert_eq!(results.len(), 3);
        let query = Product::all().where_col(|p| p.name.in_list(&["cat", "dog"]));
        let results = query.run(&conn).await.unwrap();
        assert_eq!(results.len(), 2);
    })
}

#[test]
fn should_be_able_to_select_all_products_with_there_orders() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let query = Product::all().include(|x| x.orders).order_by_asc(|x| x.id);
        let products = query.run(&conn).await.unwrap();

        // first product has 2 orders
        let p1 = products.get(0).unwrap();
        let p1_orders = p1.get(|x| x.orders);
        assert_eq!(p1_orders.len(), 2);

        // second product has 1 orders
        let p2 = products.get(1).unwrap();
        let p2_orders = p2.get(|x| x.orders);
        assert_eq!(p2_orders.len(), 1);

        // third product has 0 orders
        let p3 = products.get(2).unwrap();
        let p3_orders = p3.get(|x| x.orders);
        assert_eq!(p3_orders.len(), 0);
    })
}

#[test]
fn should_be_able_to_select_all_orders_with_there_products() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let query = Order::all().include(|x| x.product).order_by_asc(|x| x.id);
        let orders = query.run(&conn).await.unwrap();

        // From the test data:
        // order 1 and 2 point to product 1
        // order 3 point to product 2
        let o1 = orders.get(0).unwrap();
        let o1_products = o1.get(|x| x.product);
        assert_eq!(o1_products.len(), 1);
        assert_eq!(o1_products[0].id, 1);

        let o2 = orders.get(1).unwrap();
        let o2_products = o2.get(|x| x.product);
        assert_eq!(o2_products.len(), 1);
        assert_eq!(o2_products[0].id, 1);

        let o3 = orders.get(2).unwrap();
        let o3_products = o3.get(|x| x.product);
        assert_eq!(o3_products.len(), 1);
        assert_eq!(o3_products[0].id, 2);
    })
}

#[test]
fn should_be_able_to_fetch_a_single_object() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let result: DbState<Product> = Product::where_col(|p| p.id.gt(1))
            .fetch_one(&conn)
            .await
            .unwrap();
    })
}

//#[test]
//fn should_be_able_to_join_limit_and_order_all_at_once() {
//    async_std::task::block_on(async {
//        let conn = get_conn().await;
//        let q = Product::all().select(|x| x.id).join(
//            |x| x.orders,
//            Order::all()
//                .select(|o| o.id)
//                .limit(1)
//                .order_by_asc(|o| o.id),
//        );
//    })
//}
