pub mod database;
pub mod errors;
pub mod query;
pub mod table;

#[test]
fn verify_pg_test_db_connection() {
    async_std::task::block_on(async {
        let conn = testlib::postgres::conn().await.unwrap();
        assert!(!conn.is_closed(), "Error Connecting to Testing DB");
    })
}
