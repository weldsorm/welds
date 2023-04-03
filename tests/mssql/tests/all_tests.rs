use mssql_test::models::order::Order;
use mssql_test::models::product::Product;
use mssql_test::models::product_lite::Product as ProductLite;

#[derive(Default, Debug, Clone, sqlx::FromRow)]
pub struct Test {
    pub id: i32,
}

#[test]
fn test_selecting_from_mssql() {
    async_std::task::block_on(async {
        use sqlx::database::HasArguments;
        use sqlx::query::QueryAs;
        use sqlx::Arguments;
        use sqlx::Mssql;

        let sql = "SELECT id FROM welds.products where id != @p1 AND id != @p2";

        let mut args: <Mssql as HasArguments>::Arguments = Default::default();
        args.add(41);
        args.add(43);

        let q: QueryAs<Mssql, Test, <Mssql as HasArguments>::Arguments> =
            sqlx::query_as_with(sql, args);

        let conn = testlib::mssql::conn().await.unwrap();
        let pool: welds_core::database::Pool = conn.into();
        let conn = pool.as_mssql().unwrap();
        let data = q.fetch_all(conn).await.unwrap();

        assert!(data.len() > 0);
    })
}

#[test]
fn should_be_able_to_read_all_products_lite() {
    async_std::task::block_on(async {
        let conn = testlib::mssql::conn().await.unwrap();
        let pool: welds_core::database::Pool = conn.into();
        let conn = pool.as_mssql().unwrap();
        let mut q = ProductLite::all();
        let sql = q.to_sql();
        eprintln!("SQL: {}", sql);
        let all = q.run(conn).await.unwrap();

        assert_eq!(all.len(), 6, "Unexpected number of rows returned");
    })
}

#[test]
fn should_be_able_to_count_in_sql_product_lite() {
    async_std::task::block_on(async {
        let conn = testlib::mssql::conn().await.unwrap();
        let pool: welds_core::database::Pool = conn.into();
        let conn = pool.as_mssql().unwrap();
        let mut q = ProductLite::all();
        let count = q.count(conn).await.unwrap();
        assert_eq!(count, 6);
    })
}

#[test]
fn should_be_able_to_read_all_products() {
    async_std::task::block_on(async {
        let conn = testlib::mssql::conn().await.unwrap();
        let pool: welds_core::database::Pool = conn.into();
        let conn = pool.as_mssql().unwrap();
        let mut q = Product::all();
        eprintln!("SQL: {}", q.to_sql());
        let all = q.run(conn).await.unwrap();

        assert_eq!(all.len(), 6, "Unexpected number of rows returned");
    })
}

#[test]
fn should_be_able_to_filter_on_equal() {
    async_std::task::block_on(async {
        let conn = testlib::mssql::conn().await.unwrap();
        let pool: welds_core::database::Pool = conn.into();
        let conn = pool.as_mssql().unwrap();
        let mut q = Product::where_col(|x| x.price1.equal(1.10));
        eprintln!("SQL: {}", q.to_sql());
        let just_horse = q.run(conn).await.unwrap();
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
        let conn = testlib::mssql::conn().await.unwrap();
        let pool: welds_core::database::Pool = conn.into();
        let conn = pool.as_mssql().unwrap();
        let mut q = Product::where_col(|x| x.price1.lt(3.00));
        eprintln!("SQL: {}", q.to_sql());
        let data = q.run(conn).await.unwrap();
        assert_eq!(data.len(), 2, "Expected horse and dog",);
    })
}

#[test]
fn should_be_able_to_filter_on_lte() {
    async_std::task::block_on(async {
        eprintln!("1");
        let conn = testlib::mssql::conn().await.unwrap();
        eprintln!("2");
        let pool: welds_core::database::Pool = conn.into();
        eprintln!("3");
        let conn = pool.as_mssql().unwrap();
        eprintln!("4");
        let mut q = Product::where_col(|x| x.price1.lte(2.10));
        eprintln!("5");
        eprintln!("SQL: {}", q.to_sql());
        eprintln!("6");
        let data = q.run(conn).await.unwrap();
        eprintln!("7");
        assert_eq!(data.len(), 2, "Expected horse and dog");
    })
}

#[test]
fn should_be_able_to_count_in_sql() {
    async_std::task::block_on(async {
        let conn = testlib::mssql::conn().await.unwrap();
        let pool: welds_core::database::Pool = conn.into();
        let conn = pool.as_mssql().unwrap();
        let mut q = Product::where_col(|x| x.price1.lte(2.10));
        eprintln!("SQL: {}", q.to_sql());
        let count = q.count(conn).await.unwrap();
        assert_eq!(count, 2,);
    })
}

#[test]
fn should_be_able_to_limit_results_in_sql() {
    async_std::task::block_on(async {
        let conn = testlib::mssql::conn().await.unwrap();
        let pool: welds_core::database::Pool = conn.into();
        let conn = pool.as_mssql().unwrap();
        let mut q = Product::all().limit(2).offset(1);
        eprintln!("SQL: {}", q.to_sql());
        let count = q.run(conn).await.unwrap().len();
        assert_eq!(count, 2);
    })
}

#[test]
fn should_be_able_to_create_a_new_product() {
    async_std::task::block_on(async {
        let conn = testlib::mssql::conn().await.unwrap();
        let pool: welds_core::database::Pool = conn.into();
        let conn = pool.as_mssql().unwrap();
        let mut trans = conn.begin().await.unwrap();

        let mut p1 = Product::new();
        p1.name = "newyNewFace".to_owned();
        p1.description = Some("YES!".to_owned());
        // Note: creation will set the PK for the model.
        p1.save(&mut trans).await.unwrap();

        let mut q = Product::where_col(|x| x.id.equal(p1.id));
        let mut found: Vec<_> = q.run(&mut trans).await.unwrap();
        let p2 = found.pop().unwrap();
        assert_eq!(p2.name, "newyNewFace");

        trans.rollback().await.unwrap();
    })
}

#[test]
fn should_be_able_to_filter_on_relations() {
    async_std::task::block_on(async {
        let conn = testlib::mssql::conn().await.unwrap();
        let pool: welds_core::database::Pool = conn.into();
        let conn = pool.as_mssql().unwrap();
        let mut orders = Product::where_col(|x| x.id.equal(1)).map_query(|p| p.orders);
        let orders = orders.run(conn).await.unwrap();
        assert_eq!(3, orders.len());
    })
}

#[test]
fn should_be_able_to_filter_on_relations2() {
    async_std::task::block_on(async {
        let conn = testlib::mssql::conn().await.unwrap();
        let pool: welds_core::database::Pool = conn.into();
        let conn = pool.as_mssql().unwrap();
        let mut product_query = Order::all().map_query(|p| p.product);
        // Vec<_> would be simpler, but want to hard code to type for test.
        use welds_core::state::DbState;
        let products: Vec<DbState<Product>> = product_query.run(conn).await.unwrap();
        assert_eq!(2, products.len());
    })
}

#[test]
fn should_be_able_to_filter_with_relations() {
    async_std::task::block_on(async {
        let conn = testlib::mssql::conn().await.unwrap();
        let pool: welds_core::database::Pool = conn.into();
        let conn = pool.as_mssql().unwrap();
        let filter1 = Product::where_col(|x| x.id.equal(1));
        let mut order_query = Order::all();
        order_query = order_query.where_relation(|o| o.product, filter1);
        // Vec<_> would be simpler, but want to hard code to type for test.
        use welds_core::state::DbState;
        let orders: Vec<DbState<Order>> = order_query.run(conn).await.unwrap();
        assert_eq!(3, orders.len());
    })
}

#[test]
fn should_be_able_to_filter_with_relations2() {
    async_std::task::block_on(async {
        let conn = testlib::mssql::conn().await.unwrap();
        let pool: welds_core::database::Pool = conn.into();
        let conn = pool.as_mssql().unwrap();
        let filter1 = Order::where_col(|x| x.id.lte(3));
        let mut product_query = Product::all();
        product_query = product_query.where_relation(|p| p.orders, filter1);
        // Vec<_> would be simpler, but want to hard code to type for test.
        use welds_core::state::DbState;
        let orders: Vec<DbState<Product>> = product_query.run(conn).await.unwrap();
        assert_eq!(1, orders.len());
    })
}
