use sqlite_test::models::order::{Order, SmallOrder};
use sqlite_test::models::product::{BadProduct1, BadProduct2, Product};
use sqlite_test::models::StringThing;
use sqlite_test::models::{Thing1, Thing2, Thing3};
use welds::connections::sqlite_sync::SqliteClient;
use welds::connections::TransactStart;
use welds::state::{DbState, DbStatus};
use welds::Syntax;

pub mod bulk_delete;
pub mod bulk_update;
pub mod callbacks;
pub mod extra_types;
pub mod group_by;
pub mod ignores;
pub mod includes;
pub mod migrations;
pub mod select_col;
pub mod sub_query_tests;

// Get a DB connection for testing
fn get_conn() -> SqliteClient {
    conn_inner().unwrap()
}

/// Build a Connection to test the Sqlite database.
/// db is pre-seeded with contents for test.
fn conn_inner() -> Result<SqliteClient, welds::errors::WeldsError> {
    let url = "sqlite::memory:";
    let pool = welds::connections::sqlite_sync::connect(url)?;

    // Make the tables
    let schema = include_str!("../../testlib/databases/sqlite/01_create_tables.sql");
    pool.execute_script(schema)?;

    // Add Data to table
    let data = include_str!("../../testlib/databases/sqlite/02_add_test_data.sql");
    pool.execute_script(data)?;

    // Add Views
    let views = include_str!("../../testlib/databases/sqlite/03_create_views.sql");
    pool.execute_script(views)?;

    Ok(pool)
}

#[derive(Default, Debug, Clone)]
pub struct Count {
    pub count: i32,
}

#[test]
fn should_be_able_to_read_all_products() {
    let conn = get_conn();
    let q = Product::all();
    eprintln!("SQL: {}", q.to_sql(Syntax::Sqlite));
    let all = q.run(&conn).unwrap();
    assert_eq!(all.len(), 6, "Unexpected number of rows returned");
}

#[test]
fn should_be_able_to_filter_on_id() {
    let conn = get_conn();

    let q = Product::where_col(|x| x.id.equal(1));
    eprintln!("SQL: {}", q.to_sql(Syntax::Sqlite));
    let just_horse = q.run(&conn).unwrap();
    assert_eq!(
        just_horse.len(),
        1,
        "Expected to only find the horse in the test data"
    );
}

#[test]
fn should_lt() {
    let conn = get_conn();
    let q = Product::where_col(|x| x.price1.lt(2.10));
    eprintln!("SQL: {}", q.to_sql(Syntax::Sqlite));
    let data = q.run(&conn).unwrap();
    assert_eq!(data.len(), 1);
}

#[test]
fn should_be_able_to_filter_on_equal() {
    let conn = get_conn();
    let q = Product::where_col(|x| x.id.equal(1));
    eprintln!("SQL: {}", q.to_sql(Syntax::Sqlite));
    let just_horse = q.run(&conn).unwrap();
    assert_eq!(
        just_horse.len(),
        1,
        "Expected to only find the horse in the test data"
    );
}

#[test]
fn should_be_able_to_filter_on_lt() {
    let conn = get_conn();
    let q = Product::where_col(|x| x.price1.lt(3.00));
    eprintln!("SQL: {}", q.to_sql(Syntax::Sqlite));
    let data = q.run(&conn).unwrap();
    assert_eq!(data.len(), 2, "Expected horse and dog",);
}

#[test]
fn should_be_able_to_filter_on_lte() {
    let conn = get_conn();
    let q = Product::where_col(|x| x.id.lte(2));
    eprintln!("SQL: {}", q.to_sql(Syntax::Sqlite));
    let data = q.run(&conn).unwrap();
    assert_eq!(data.len(), 2, "Expected horse and dog",);
}

#[test]
fn should_be_able_to_filter_with_nulls() {
    let conn = get_conn();
    // is null
    let q1 = Product::where_col(|x| x.price1.equal(None));
    eprintln!("SQL_1: {}", q1.to_sql(Syntax::Sqlite));
    let data1 = q1.run(&conn).unwrap();
    assert_eq!(data1.len(), 0, "Expected All",);
    // is not null
    let q1 = Product::where_col(|x| x.price1.not_equal(None));
    eprintln!("SQL_2: {}", q1.to_sql(Syntax::Sqlite));
    let data1 = q1.run(&conn).unwrap();
    assert_eq!(data1.len(), 6, "Expected All",);
}

#[test]
fn should_be_able_to_count_in_sql() {
    let conn = get_conn();
    let q = Product::where_col(|x| x.price1.lte(2.15));
    eprintln!("SQL: {}", q.to_sql_count(Syntax::Sqlite));
    let count = q.count(&conn).unwrap();
    assert_eq!(count, 2,);
}

#[test]
fn should_be_able_to_limit_results_in_sql() {
    let conn = get_conn();
    let q = Product::all().limit(2).offset(1);
    eprintln!("SQL: {}", q.to_sql(Syntax::Sqlite));
    let count = q.run(&conn).unwrap().len();
    assert_eq!(count, 2);
}

#[test]
fn should_be_able_to_crud_with_small_int() {
    let client = get_conn();
    let trans = client.begin().unwrap();

    let mut p1 = Product::new();
    p1.save(&trans).unwrap();
    let mut p2 = Product::new();
    p2.save(&trans).unwrap();
    assert!(p1.id != 0);
    assert!(p2.id != 0);

    let mut s = SmallOrder::new();
    s.product_id = p1.id;
    s.save(&trans).unwrap();
    let s = SmallOrder::find_by_id(&trans, s.id).expect("db err");
    let mut s = s.expect("new SmallOrder not found");
    assert_eq!(s.product_id, p1.id);
    s.product_id = p2.id;
    s.save(&trans).unwrap();
    let s = SmallOrder::find_by_id(&trans, s.id).expect("db err");
    let mut s = s.expect("new SmallOrder not found");
    assert_eq!(s.product_id, p2.id);
    s.delete(&trans).expect("delete db err");
    let s_none = SmallOrder::find_by_id(&trans, s.id).expect("db err");
    assert!(s_none.is_none());

    trans.rollback().unwrap();
}

#[test]
fn should_be_able_to_order_by_id() {
    let conn = get_conn();
    let q = Product::all().order_by_asc(|x| x.id);
    eprintln!("SQL: {}", q.to_sql(Syntax::Sqlite));
    let all = q.run(&conn).unwrap();
    let ids: Vec<i32> = all.iter().map(|x| x.id).collect();
    let mut ids_sorted = ids.clone();
    ids_sorted.sort();
    assert_eq!(ids, ids_sorted);
}

#[test]
fn should_be_able_to_update_a_product() {
    let conn = get_conn();
    let trans = conn.begin().unwrap();

    let q = Product::all().limit(1);
    let mut found: Vec<_> = q.run(&trans).unwrap();
    let mut p1 = found.pop().unwrap();
    p1.name = "Test1".to_owned();
    p1.save(&trans).unwrap();

    let q = Product::where_col(|x| x.id.equal(p1.id));
    let mut found: Vec<_> = q.run(&trans).unwrap();
    let p2 = found.pop().unwrap();
    assert_eq!(p2.name, "Test1");

    trans.rollback().unwrap();
}

#[test]
fn should_be_able_to_create_a_new_product() {
    let conn = get_conn();
    let trans = conn.begin().unwrap();

    let mut p1 = Product::new();
    p1.name = "newyNewFace".to_owned();
    p1.description = Some("YES!".to_owned());
    // Note: creation will set the PK for the model.
    p1.save(&trans).unwrap();

    let q = Product::where_col(|x| x.id.equal(p1.id));
    let mut found: Vec<_> = q.run(&trans).unwrap();
    let p2 = found.pop().unwrap();
    assert_eq!(p2.name, "newyNewFace");
    assert!(p2.id != 0, "Expected new ID");

    let count = Product::where_col(|x| x.id.equal(p1.id))
        .count(&trans)
        .unwrap();
    assert_eq!(count, 1);

    trans.rollback().unwrap();
}

#[test]
fn should_be_able_to_scan_for_all_tables() {
    let conn = get_conn();
    let tables = welds::detect::find_all_tables(&conn).unwrap();
    assert_eq!(19, tables.len());
}

#[test]
fn a_model_should_be_able_to_verify_its_schema_missing_table() {
    let conn = get_conn();
    let issues = welds::check::schema::<BadProduct1>(&conn).unwrap();
    assert_eq!(issues.len(), 1);
    let issue = &issues[0];
    assert_eq!(issue.kind, welds::check::Kind::MissingTable);
}

#[test]
fn a_model_should_be_able_to_verify_its_schema_missing_column() {
    let conn = get_conn();
    let issues = welds::check::schema::<BadProduct2>(&conn).unwrap();
    // NOTE: a column name changed so it is added on the model and removed in the db giving two warnings
    for issue in &issues {
        eprintln!("{}", issue);
    }
    assert_eq!(issues.len(), 7);
}

#[test]
fn should_be_able_to_bulk_delete() {
    let conn = get_conn();
    let trans = conn.begin().unwrap();
    let p1 = Product::all().limit(1).run(&trans).unwrap().pop().unwrap();
    let mut order = Order::new();
    order.product_id = p1.id;
    order.save(&trans).unwrap();
    let q = Product::all().map_query(|p| p.orders);
    let count = q.count(&trans).unwrap();
    q.delete(&trans).unwrap();
    assert!(count > 0);
    trans.rollback().unwrap();
}

#[test]
fn should_be_able_to_bulk_delete2() {
    let conn = get_conn();
    let trans = conn.begin().unwrap();
    let p1 = Product::all().limit(1).run(&trans).unwrap().pop().unwrap();
    let mut order = Order::new();
    order.product_id = p1.id;
    order.save(&trans).unwrap();
    let q = Order::all().where_col(|x| x.id.gt(0));
    let count = q.count(&trans).unwrap();
    q.delete(&trans).unwrap();
    assert!(count > 0);
    trans.rollback().unwrap();
}

#[test]
fn should_be_able_to_bulk_update() {
    let conn = get_conn();
    let q = Order::all()
        .where_col(|x| x.code.equal(None))
        .set(|x| x.code, "test");
    let sql = q.to_sql(Syntax::Sqlite);
    eprintln!("SQL: {}", sql);
    q.run(&conn).unwrap();
}

#[test]
fn should_be_able_to_bulk_update2() {
    let conn = get_conn();
    let q = Product::all()
        .map_query(|p| p.orders)
        .set(|x| x.code, "test2");
    let sql = q.to_sql(Syntax::Sqlite);
    eprintln!("SQL: {}", sql);
    q.run(&conn).unwrap();
}

#[test]
fn should_be_able_to_bulk_update_by_set_col() {
    let conn = get_conn();
    let q = Product::all()
        .map_query(|p| p.orders)
        .where_col(|c| c.id.equal(2342534))
        .set_col(|x| x.code.equal("test2"));
    let sql = q.to_sql(Syntax::Postgres);
    eprintln!("SQL: {}", sql);
    q.run(&conn).unwrap();
}

#[test]
fn should_be_able_to_limit_deletes() {
    let conn = get_conn();
    let trans = conn.begin().unwrap();
    for _ in 0..100 {
        Thing1::new().save(&trans).unwrap();
    }
    let count_before = Thing1::all().count(&trans).unwrap();
    Thing1::all()
        .order_by_desc(|x| x.id)
        .limit(1)
        .delete(&trans)
        .unwrap();
    let count_after = Thing1::all().count(&trans).unwrap();
    assert_eq!(count_before - 1, count_after);
    trans.rollback().unwrap();
}

#[test]
fn should_only_update_limited_rows_if_limit_is_in_query() {
    let conn = get_conn();
    let trans = conn.begin().unwrap();
    for _ in 0..10 {
        Thing2::new().save(&trans).unwrap();
    }
    let update_statment = Thing2::all()
        .where_col(|x| x.id.gt(0))
        .order_by_desc(|x| x.id)
        .limit(1)
        .set(|x| x.value, "HAS_VALUE");

    let sql = update_statment.to_sql(Syntax::Sqlite);
    update_statment.run(&trans).unwrap();
    eprintln!("SQL: {}", sql);

    let count = Thing2::where_col(|x| x.value.equal("HAS_VALUE"))
        .count(&trans)
        .unwrap();
    assert_eq!(count, 1);
    trans.rollback().unwrap();
}

#[test]
fn should_be_able_to_bulk_insert() {
    let conn = get_conn();
    let trans = conn.begin().unwrap();
    let things: Vec<_> = (0..3000)
        .map(|x| Thing3 {
            id: 0,
            value: format!("Bulk_Insert: {}", x),
        })
        .collect();
    welds::query::insert::bulk_insert(&trans, &things).unwrap();
    trans.rollback().unwrap();
}

#[test]
fn should_be_able_to_create_a_model_with_a_string_id() {
    let conn = get_conn();
    let mut thing = DbState::new_uncreated(StringThing {
        id: "test".to_owned(),
        value: "test".to_owned(),
    });
    thing.save(&conn).unwrap();
    assert_eq!(thing.db_status(), DbStatus::NotModified);
    let found = StringThing::find_by_id(&conn, "test".to_owned()).unwrap();
    assert!(found.is_some());
}

#[test]
fn should_be_able_to_set_a_nullable_value_to_null() {
    let conn = get_conn();
    let trans = conn.begin().unwrap();

    Order::all()
        .where_col(|x| x.code.equal("333"))
        .set_null(|x| x.code)
        .run(&trans)
        .unwrap();

    trans.rollback().unwrap();
}

#[test]
fn should_be_able_to_write_a_custom_set() {
    use welds::query::builder::ManualParam;
    let params = ManualParam::new().push(1);
    let conn = get_conn();
    let q = Product::all()
        .map_query(|p| p.orders)
        .where_col(|c| c.id.equal(2342534))
        .set_manual(|x| x.product_id, "product_id + ?", params);
    let sql = q.to_sql(Syntax::Sqlite);
    eprintln!("SQL: {}", sql);
    q.run(&conn).unwrap();
}

#[test]
fn should_be_able_to_write_a_custom_set3() {
    let conn = get_conn();
    let q = Product::all()
        .map_query(|p| p.orders)
        .where_col(|c| c.id.equal(2342534))
        .set_manual(|x| x.product_id, "product_id + ? + ?", (42, 20.0));
    let sql = q.to_sql(Syntax::Sqlite);
    eprintln!("SQL: {}", sql);
    q.run(&conn).unwrap();
}

#[test]
fn should_be_able_to_write_a_custom_set2() {
    let conn = get_conn();
    let q = Product::all()
        .map_query(|p| p.orders)
        .where_col(|c| c.id.equal(2342534))
        .set_manual(|x| x.product_id, "product_id + ?", (2,));
    let sql = q.to_sql(Syntax::Sqlite);
    eprintln!("SQL: {}", sql);
    q.run(&conn).unwrap();
}

#[test]
fn should_be_able_to_filter_by_multiple_values() {
    let conn = get_conn();
    let query = Product::all().where_col(|p| p.id.in_list(&[2, 3, 4]));
    let results = query.run(&conn).unwrap();
    assert_eq!(results.len(), 3);
    let query = Product::all().where_col(|p| p.name.in_list(&["cat", "dog"]));
    let results = query.run(&conn).unwrap();
    assert_eq!(results.len(), 2);
}

#[test]
fn should_be_able_to_select_all_products_with_there_orders() {
    let conn = get_conn();
    let query = Product::all().include(|x| x.orders).order_by_asc(|x| x.id);
    let products = query.run(&conn).unwrap();

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
}

#[test]
fn should_be_able_to_select_all_orders_with_there_products() {
    let conn = get_conn();
    let query = Order::all().include(|x| x.product).order_by_asc(|x| x.id);
    let orders = query.run(&conn).unwrap();

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
}

#[test]
fn should_be_able_to_fetch_a_single_object() {
    let conn = get_conn();
    let _product: DbState<Product> = Product::where_col(|p| p.id.gt(1)).fetch_one(&conn).unwrap();
}

// // unstable-api is not enabled
// #[test]
// fn should_be_able_to_select_hourse_or_dog() {
//     use welds::query::clause::or;
//     let conn = get_conn();
//     use sqlite_test::models::product::ProductSchema;
//
//     // verify pulling out lambda into variable
//     let clause = |x: ProductSchema| or(x.name.like("horse"), x.name.like("dog"));
//     let q = Product::all().where_col(clause);
//
//     eprintln!("SQL: {}", q.to_sql(Syntax::Sqlite));
//     let data = q.run(&conn).unwrap();
//     assert_eq!(data.len(), 2, "Expected horse and dog",);
//
//     // verify inline clause
//     let q2 = Product::all().where_col(|x| or(x.name.like("horse"), x.name.like("dog")));
//     eprintln!("SQL: {}", q2.to_sql(Syntax::Sqlite));
//     let data = q2.run(&conn).unwrap();
//     assert_eq!(data.len(), 2, "Expected horse and dog",);
// }
