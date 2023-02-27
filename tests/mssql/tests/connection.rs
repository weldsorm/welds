#[test]
fn should_be_able_to_connect() {
    async_std::task::block_on(async {
        let conn = testlib::mssql::conn().await.unwrap();
        assert!(!conn.is_closed(), "Connection is Closed");
    })
}
