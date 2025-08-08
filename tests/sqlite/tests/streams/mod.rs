use async_std::stream::StreamExt;
use sqlite_test::models::product::Product;
use welds::prelude::*;

use super::get_conn;

const SQL_RANGE_10: &str = "SELECT n AS value FROM ( SELECT 1 AS n UNION ALL SELECT 2 UNION ALL SELECT 3 UNION ALL SELECT 4 UNION ALL SELECT 5 UNION ALL SELECT 6 UNION ALL SELECT 7 UNION ALL SELECT 8 UNION ALL SELECT 9 UNION ALL SELECT 10) AS numbers";
const SQL_VALUE: &str = "SELECT ?";

#[test]
fn should_be_able_to_stream_rows() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let mut stream = conn.stream(SQL_RANGE_10, &[]).await;
        let mut rows: Vec<_> = Vec::default();
        while let Some(r) = stream.next().await {
            rows.push(r);
        }
        assert_eq!(rows.len(), 10);
    })
}

#[test]
fn should_be_able_to_stream_rows_values() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let expected = 44_i64;
        let mut stream = conn.stream(SQL_VALUE, &[&expected]).await;
        let row = stream.next().await.unwrap().unwrap();
        let acual: i64 = row.get_by_position(0).unwrap();
        assert_eq!(expected, acual);
    })
}

#[test]
fn should_be_able_to_stream_into_models() {
    async_std::task::block_on(async {
        let conn = get_conn().await;
        let expected = Product::all().run(&conn).await.unwrap().into_inners();

        // stream the query into a vec
        let query = Product::all();
        let mut stream = query.stream(&conn).await;
        let mut actual = Vec::default();
        while let Some(p) = stream.next().await {
            actual.push(p.unwrap());
        }

        assert_eq!(expected.len(), actual.len());
        for (e, a) in expected.iter().zip(actual) {
            assert_eq!(e, &a);
        }
    })
}
