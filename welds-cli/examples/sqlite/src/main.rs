use welds_core::database::Pool;
mod models;
use models::order::Order;
use models::people::People;
//use welds_core::query::clause::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let connection_string = "sqlite://sample_db.sqlite";
    let conn = welds_core::database::connect_with_connection_string(connection_string).await;
    let conn = conn.unwrap();
    test_sql(&conn);
    test_run(&conn).await;
}

fn test_sql(pool: &Pool) {
    //let mut x = Order::all();
    //x = x.where_col(|x| x.id.not_equal(None));
    //x = x.where_col(|x| x.name.not_equal("test"));
    //x = x.where_col(|x| x.name.not_equal(None));
    //let sql = x.to_sql(pool);
    //dbg!(sql);
}

async fn test_run(pool: &Pool) {
    let mut q = Order::all();
    //q = q.where_col(|x| x.id.equal(1));
    q = q.where_col(|x| x.price.equal(999.0));
    //q = q.where_col(|x| x.id.not_equal(None));
    //q = q.where_col(|x| x.name.not_equal("test"));
    //q = q.where_col(|x| x.name.not_equal(None));
    let data = q.run(pool).await.unwrap();
    dbg!(data);
}
