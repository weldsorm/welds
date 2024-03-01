use postgres_test::models::enums::Color;
use postgres_test::models::order::Order;
use postgres_test::models::other::Other;
use postgres_test::models::product::{BadProductColumns, BadProductMissingTable, Product};
use postgres_test::models::table_with_array::TableWithArray;
use postgres_test::models::Thing1;
use welds::connections::postgres::PostgresClient;
use welds::connections::TransactStart;
use welds::Syntax;

mod migrations;

async fn get_conn() -> PostgresClient {
    let conn = testlib::postgres::conn().await.unwrap();
    let client: PostgresClient = conn.into();
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
        eprintln!("SQL: {}", q.to_sql(Syntax::Postgres));
        let all = q.run(&conn).await.unwrap();
        assert_eq!(all.len(), 6, "Unexpected number of rows returned");
    })
}

#[test]
fn should_be_able_to_filter_on_equal() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let q = Product::where_col(|x| x.price_1.equal(1.10));
        eprintln!("SQL: {}", q.to_sql(Syntax::Postgres));
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
        let q = Product::where_col(|x| x.price_1.lt(3.00));
        eprintln!("SQL: {}", q.to_sql(Syntax::Postgres));
        let data = q.run(&conn).await.unwrap();
        assert_eq!(data.len(), 2, "Expected horse and dog",);
    })
}

#[test]
fn should_be_able_to_filter_on_lte() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let q = Product::where_col(|x| x.price_1.lte(2.10));
        eprintln!("SQL: {}", q.to_sql(Syntax::Postgres));
        let data = q.run(&conn).await.unwrap();
        assert_eq!(data.len(), 2, "Expected horse and dog",);
    })
}

#[test]
fn should_be_able_to_filter_with_nulls() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        // is null
        let q1 = Product::where_col(|x| x.price_1.equal(None));
        eprintln!("SQL_1: {}", q1.to_sql(Syntax::Postgres));
        let data1 = q1.run(&conn).await.unwrap();
        assert_eq!(data1.len(), 0, "Expected All",);
        // is not null
        let q1 = Product::where_col(|x| x.price_1.not_equal(None));
        eprintln!("SQL_2: {}", q1.to_sql(Syntax::Postgres));
        let data1 = q1.run(&conn).await.unwrap();
        assert_eq!(data1.len(), 6, "Expected All",);
    })
}

#[test]
fn should_be_able_to_count_in_sql() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let q = Product::where_col(|x| x.price_1.lte(2.10));
        eprintln!("SQL: {}", q.to_sql(Syntax::Postgres));
        let count = q.count(&conn).await.unwrap();
        assert_eq!(count, 2,);
    })
}

#[test]
fn should_be_able_to_limit_results_in_sql() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let q = Product::all().limit(2).offset(1);
        eprintln!("SQL: {}", q.to_sql(Syntax::Postgres));
        let count = q.run(&conn).await.unwrap().len();
        assert_eq!(count, 2);
    })
}

#[test]
fn should_be_able_to_order_by_id() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let q = Product::all().order_by_asc(|x| x.product_id);
        eprintln!("SQL: {}", q.to_sql(Syntax::Postgres));
        let all = q.run(&conn).await.unwrap();
        let ids: Vec<i32> = all.iter().map(|x| x.product_id).collect();
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

        let q = Product::where_col(|x| x.product_id.equal(p1.product_id));
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

        let q = Product::where_col(|x| x.product_id.equal(p1.product_id));
        let mut found: Vec<_> = q.run(&mut trans).await.unwrap();
        let p2 = found.pop().unwrap();
        assert_eq!(p2.name, "newyNewFace");

        trans.rollback().await.unwrap();
    })
}

#[test]
fn should_be_able_to_drop_down_to_sqlx() {
    async_std::task::block_on(async {
        //setup
        let client = get_conn().await;
        let conn = client.as_sqlx_pool();
        // Build some args to send to the database
        use sqlx::{postgres::PgArguments, Arguments};
        let mut args: PgArguments = Default::default();
        args.add(1_i32);
        // Go run a query from the database.
        let sql = "SELECT * FROM products where product_id > $1";
        let all_but_one: Vec<Product> = sqlx::query_as_with(sql, args)
            .fetch_all(conn)
            .await
            .unwrap();
        assert_eq!(all_but_one.len(), 5);
    })
}

#[test]
fn should_be_able_to_run_raw_sql() {
    async_std::task::block_on(async {
        //setup
        let conn = get_conn().await;

        // Go run a query from the database.
        let sql = "SELECT * FROM products";

        let args = Vec::default();
        let all = Product::from_raw_sql(sql, &args, &conn).await.unwrap();

        assert_eq!(all.len(), 6);
    })
}

#[test]
fn should_be_able_to_crud_orders() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let mut trans = conn.begin().await.unwrap();
        let mut o = Order::new();
        o.quantity = Some(9942);
        o.save(&mut trans).await.unwrap();
        let created = Order::all()
            .limit(1)
            .order_by_desc(|x| x.id)
            .run(&mut trans)
            .await
            .unwrap();
        let created2 = Order::where_col(|x| x.quantity.gt(9941))
            .run(&mut trans)
            .await
            .unwrap();
        let created1 = &created[0];
        let created2 = &created2[0];
        assert_eq!(o.id, created1.id);
        assert_eq!(created1.quantity, Some(9942));
        assert_eq!(o.id, created2.id);
        assert_eq!(created2.quantity, Some(9942));
        trans.rollback().await.unwrap();
    })
}

#[test]
fn should_be_able_to_delete_a_product() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let mut trans = conn.begin().await.unwrap();

        let id = 6;
        let original_total = Product::all().count(&mut trans).await.unwrap();
        let mut product = Product::find_by_id(&mut trans, id).await.unwrap().unwrap();
        product.delete(&mut trans).await.unwrap();
        let new_total = Product::all().count(&mut trans).await.unwrap();

        assert_eq!(new_total, original_total - 1);
        trans.rollback().await.unwrap();
    })
}

#[test]
fn should_be_able_to_find_like() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        //build the queries
        let like = Product::where_col(|x| x.name.like("%Horse%"));
        let ilike = Product::where_col(|x| x.name.ilike("%Horse%"));
        eprintln!("SQL: {}", ilike.to_sql(Syntax::Postgres));
        let not_like = Product::where_col(|x| x.name.not_like("%Horse%"));
        let not_ilike = Product::where_col(|x| x.name.not_ilike("%Horse%"));
        //run the queries
        let like = like.run(&conn).await.unwrap();
        let ilike = ilike.run(&conn).await.unwrap();
        let not_like = not_like.run(&conn).await.unwrap();
        let not_ilike = not_ilike.run(&conn).await.unwrap();
        //validate data
        assert_eq!(like.len(), 0, "like");
        assert_eq!(ilike.len(), 1, "ilike");
        assert_eq!(not_like.len(), 6, "not like");
        assert_eq!(not_ilike.len(), 5, "not ilike");
    })
}

#[test]
fn should_be_able_to_filter_on_relations() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let orders = Product::where_col(|x| x.name.like("%horse%")).map_query(|p| p.order);
        let orders = orders.run(&conn).await.unwrap();
        assert_eq!(2, orders.len());
    })
}

#[test]
fn should_be_able_to_filter_on_relations2() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let product_query = Order::all().map_query(|p| p.product);
        // Vec<_> would be simpler, but want to hard code to type for test.
        use welds::state::DbState;
        let products: Vec<DbState<Product>> = product_query.run(&conn).await.unwrap();
        assert_eq!(2, products.len());
    })
}

#[test]
fn should_be_able_to_filter_with_relations() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let filter1 = Product::where_col(|x| x.product_id.equal(1));
        let mut order_query = Order::all();
        order_query = order_query.where_relation(|o| o.product, filter1);
        // Vec<_> would be simpler, but want to hard code to type for test.
        use welds::state::DbState;
        let orders: Vec<DbState<Order>> = order_query.run(&conn).await.unwrap();
        assert_eq!(2, orders.len());
    })
}

#[test]
fn should_be_able_to_filter_with_relations2() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let filter1 = Order::where_col(|x| x.id.lte(3));
        let mut product_query = Product::all();
        product_query = product_query.where_relation(|p| p.order, filter1);
        // Vec<_> would be simpler, but want to hard code to type for test.
        use welds::state::DbState;
        let orders: Vec<DbState<Product>> = product_query.run(&conn).await.unwrap();
        assert_eq!(2, orders.len());
    })
}

#[test]
fn should_be_able_to_scan_for_all_tables() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let tables = welds::detect::find_tables(&conn).await.unwrap();
        assert!(tables.len() > 14);
    })
}

#[test]
fn should_be_able_to_scan_for_views() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let mut tables_and_view = welds::detect::find_tables(&conn).await.unwrap();
        use welds::detect::DataType::View;
        let views: Vec<_> = tables_and_view
            .drain(..)
            .filter(|x| x.ty() == View)
            .collect();
        assert_eq!(1, views.len());
    })
}

#[test]
fn should_be_able_to_save_load_obj_with_db_enum_type() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let trans = conn.begin().await.unwrap();

        let start_count = Other::all().count(&trans).await.unwrap();
        let mut tmp = Other::new();
        tmp.colour = Color::Blue;
        tmp.save(&trans).await.unwrap();

        let count = Other::all().count(&trans).await.unwrap();
        assert_eq!(start_count + 1, count);

        let loaded = Other::find_by_id(&trans, tmp.id).await.unwrap().unwrap();
        assert_eq!(*tmp, *loaded);
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
        assert_eq!(issues.len(), 5);
    })
}

#[test]
fn should_be_able_to_bulk_delete() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let trans = conn.begin().await.unwrap();
        let q = Order::where_col(|x| x.id.gt(0));
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
        let sql = q.to_sql(Syntax::Postgres);
        eprintln!("SQL: {}", sql);
        q.run(&conn).await.unwrap();
    })
}

#[test]
fn should_be_able_to_bulk_update2() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let q = Product::all()
            .map_query(|p| p.order)
            .set(|x| x.code, "test2");
        let sql = q.to_sql(Syntax::Postgres);
        eprintln!("SQL: {}", sql);
        q.run(&conn).await.unwrap();
    })
}

#[test]
fn should_be_able_to_bulk_insert_1() {
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
fn should_be_able_to_crud_table_with_arrays() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let trans = conn.begin().await.unwrap();

        let mut obj = TableWithArray::new();
        obj.numbers = Some(vec![]);
        obj.save(&trans).await.unwrap();

        let loaded1 = TableWithArray::find_by_id(&trans, obj.id)
            .await
            .unwrap()
            .unwrap();

        assert!(loaded1.numbers.as_ref().unwrap().is_empty());

        let nums = vec![1, 2, 3, 4];
        obj.numbers = Some(nums.clone());
        obj.save(&trans).await.unwrap();

        let loaded2 = TableWithArray::find_by_id(&trans, obj.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(loaded2.numbers.as_ref().unwrap(), &nums);

        trans.rollback().await.unwrap();
    })
}

#[test]
fn array_table_should_detect_array_type_as_array() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        // Get the table def
        let tabledef = welds::detect::find_table(Some("alt"), "table_with_arrays", &conn)
            .await
            .unwrap()
            .unwrap();
        // Get the column def
        let array_column = tabledef
            .columns()
            .iter()
            .find(|&c| c.name() == "numbers")
            .unwrap();
        // make sure we are reporting the array type as an array
        assert_eq!(array_column.ty(), "INT4[]");
    })
}

#[test]
fn should_be_able_to_check_the_schema() {
    async_std::task::block_on(async {
        let conn = get_conn().await;

        use postgres_test::models::*;

        let issues = welds::check::schema::<table_with_array::TableWithArray>(&conn)
            .await
            .unwrap();
        for issue in &issues {
            eprintln!("{}", issue);
        }
        assert!(issues.is_empty(), "{:?}", issues);
    })
}
