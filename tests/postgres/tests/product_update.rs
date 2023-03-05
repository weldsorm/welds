use postgres_test::models::product::Product;
use sqlx::Executor;
use sqlx::Postgres;
use sqlx::Transaction;

#[derive(Default, Debug, Clone, sqlx::FromRow)]
pub struct Count {
    pub count: i32,
}

#[test]
fn should_be_able_to_update_a_product() {
    async_std::task::block_on(async {
        let conn = testlib::postgres::conn().await.unwrap();
        let pool: welds_core::database::Pool = conn.into();
        let conn = pool.as_postgres().unwrap();
        let mut trans = conn.begin().await.unwrap();

        //<trans as sqlx::Executor<Database = Postgres>>

        //let sql = "SELECT count(*) as count FROM products";
        //use sqlx::database::HasArguments;
        //use sqlx::query::QueryAs;
        //let q: QueryAs<Postgres, Count, <Postgres as HasArguments>::Arguments> =
        //    sqlx::query_as(sql);
        //let r = q.fetch_one(&mut trans).await.unwrap();

        let mut q = Product::all().limit(1);
        let p = q.run(&mut trans).await.unwrap();

        trans.rollback().await.unwrap();
    })
}
