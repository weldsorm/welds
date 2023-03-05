use postgres_test::models::product::Product;

#[test]
fn should_be_able_to_update_a_product() {
    async_std::task::block_on(async {
        let conn = testlib::postgres::conn().await.unwrap();
        let pool: welds_core::database::Pool = conn.into();
        let conn = pool.as_postgres().unwrap();
        let trans = conn.begin().await.unwrap();

        //let p = Product::all().limit(1).run(&trans).await.unwrap();

        trans.rollback().await.unwrap();
    })
}
