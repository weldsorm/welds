use postgres_test::models::product::Product;

#[test]
fn should_be_able_to_read_all_products() {
    async_std::task::block_on(async {
        let conn = testlib::postgres::conn().await.unwrap();
        let pool: welds_core::database::Pool = conn.into();
        let conn = pool.as_postgres().unwrap();
        let mut q = Product::all();
        eprintln!("SQL: {}", q.to_sql());
        let all = q.run(conn).await.unwrap();
        assert_eq!(all.len(), 6, "Unexpected number of rows returned");
    })
}

#[test]
fn should_be_able_to_filter_on_equal() {
    async_std::task::block_on(async {
        let conn = testlib::postgres::conn().await.unwrap();
        let pool: welds_core::database::Pool = conn.into();
        let conn = pool.as_postgres().unwrap();
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
        let conn = testlib::postgres::conn().await.unwrap();
        let pool: welds_core::database::Pool = conn.into();
        let conn = pool.as_postgres().unwrap();
        let mut q = Product::where_col(|x| x.price1.lt(3.00));
        eprintln!("SQL: {}", q.to_sql());
        let data = q.run(&conn).await.unwrap();
        assert_eq!(data.len(), 2, "Expected horse and dog",);
    })
}

#[test]
fn should_be_able_to_filter_on_lte() {
    async_std::task::block_on(async {
        let conn = testlib::postgres::conn().await.unwrap();
        let pool: welds_core::database::Pool = conn.into();
        let conn = pool.as_postgres().unwrap();
        let mut q = Product::where_col(|x| x.price1.lte(2.10));
        eprintln!("SQL: {}", q.to_sql());
        let data = q.run(&conn).await.unwrap();
        assert_eq!(data.len(), 2, "Expected horse and dog",);
    })
}

#[test]
fn should_be_able_to_filter_with_nulls() {
    async_std::task::block_on(async {
        let conn = testlib::postgres::conn().await.unwrap();
        let pool: welds_core::database::Pool = conn.into();
        let conn = pool.as_postgres().unwrap();
        // is null
        let mut q1 = Product::where_col(|x| x.price1.equal(None));
        eprintln!("SQL_1: {}", q1.to_sql());
        let data1 = q1.run(&conn).await.unwrap();
        assert_eq!(data1.len(), 0, "Expected All",);
        // is not null
        let mut q1 = Product::where_col(|x| x.price1.not_equal(None));
        eprintln!("SQL_2: {}", q1.to_sql());
        let data1 = q1.run(&conn).await.unwrap();
        assert_eq!(data1.len(), 6, "Expected All",);
    })
}

#[test]
fn should_be_able_to_count_in_sql() {
    async_std::task::block_on(async {
        let conn = testlib::postgres::conn().await.unwrap();
        let pool: welds_core::database::Pool = conn.into();
        let conn = pool.as_postgres().unwrap();
        let mut q = Product::where_col(|x| x.price1.lte(2.10));
        eprintln!("SQL: {}", q.to_sql());
        let count = q.count(&conn).await.unwrap();
        assert_eq!(count, 2,);
    })
}
