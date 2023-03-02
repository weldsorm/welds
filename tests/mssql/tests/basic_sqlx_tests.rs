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
