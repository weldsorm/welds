use mssql_test::models::product_lite::Product;

#[test]
fn should_be_able_to_read_all_products() {
    async_std::task::block_on(async {
        let conn = testlib::mssql::conn().await.unwrap();
        let pool: welds_core::database::Pool = conn.into();
        let conn = pool.as_mssql().unwrap();
        let mut q = Product::all();
        let sql = q.to_sql();
        eprintln!("SQL: {}", sql);
        let all = q.run(conn).await.unwrap();

        assert_eq!(all.len(), 6, "Unexpected number of rows returned");
    })
}


#[test]
fn shoudl_be_able_to_count_in_sql() {
    async_std::task::block_on(async {
        let conn = testlib::mssql::conn().await.unwrap();
        let pool: welds_core::database::Pool = conn.into();
        let conn = pool.as_mssql().unwrap();
        let mut q = Product::all();
        let count = q.count(&conn).await.unwrap();
        assert_eq!(count, 6);
    })
}

