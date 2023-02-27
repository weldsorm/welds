use postgres_test::models::product::Product;

#[test]
fn should_be_able_to_read_all_products() {
    async_std::task::block_on(async {
        let conn = testlib::postgres::conn().await.unwrap();
        let pool: welds_core::database::Pool = conn.into();
        let conn = pool.as_postgres().unwrap();
        let all = Product::all().run(conn).await.unwrap();
        assert_eq!(all.len(), 6, "Unexpected number of rows returned");
    })
}

#[test]
fn should_be_able_to_filter_on_equal() {
    async_std::task::block_on(async {
        let conn = testlib::postgres::conn().await.unwrap();
        let pool: welds_core::database::Pool = conn.into();
        let conn = pool.as_postgres().unwrap();
        let just_horse = Product::where_col(|x| x.price1.equal(1.10))
            .run(conn)
            .await
            .unwrap();
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

        let sql = q.to_sql();
        let data = q.run(&conn).await.unwrap();
        assert_eq!(
            data.len(),
            2,
            "Expected horse and dog: \n{} \n\n{:?}",
            sql,
            dbg!(data)
        );
    })
}

#[test]
fn should_be_able_to_filter_on_lte() {
    async_std::task::block_on(async {
        let conn = testlib::postgres::conn().await.unwrap();
        let pool: welds_core::database::Pool = conn.into();
        let conn = pool.as_postgres().unwrap();
        let mut q = Product::where_col(|x| x.price1.lte(2.10));
        let sql = q.to_sql();
        let data = q.run(&conn).await.unwrap();
        assert_eq!(
            data.len(),
            2,
            "Expected horse and dog: \n{} \n\n{:?}",
            sql,
            dbg!(data)
        );
    })
}
