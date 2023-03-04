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
        let count = q.count(&conn).await.unwrap();
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
        let data = q.run(&conn).await.unwrap();
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
        let data = q.run(&conn).await.unwrap();
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
        let count = q.count(&conn).await.unwrap();
        assert_eq!(count, 2,);
    })
}
