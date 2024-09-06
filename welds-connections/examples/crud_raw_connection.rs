use welds_connections::{Client, TransactStart};

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    // create the sqlite file if it doesn't exist
    let _ = std::fs::File::create("./test.sqlite")?;

    let url = "sqlite://./test.sqlite";
    let conn = welds_connections::sqlite::connect(url).await.unwrap();

    //let url = "server=127.0.0.1;user id=sa;password=THEBESTPASSWORDEVER;TrustServerCertificate=true;";
    //let conn = welds_connections::mssql::connect(url).await.unwrap();

    //let url = "postgres://postgres:password@localhost:5432";
    //let conn = welds_connections::postgres::connect(url).await.unwrap();

    //let url = "mysql://root:welds!123@localhost:3306/weldstests";
    //let conn = welds_connections::mysql::connect(url).await.unwrap();

    println!("CONNECTED!");

    init_tables(&conn).await;
    in_a_transaction(&conn).await;
    println!("DID THINGS!");
    Ok(())
}

async fn in_a_transaction<T>(client: &T)
where
    T: Client + TransactStart,
{
    let trans = client.begin().await.unwrap();
    do_things(&trans).await;
    //let _ = trans.rollback().await;
    let _ = trans.commit().await;
}

async fn init_tables<T>(client: &T)
where
    T: Client,
{
    let sql = "DROP TABLE test";
    let _ = client.execute(sql, &[]).await;
    let sql = "CREATE TABLE test (name Text)";
    let _ = client.execute(sql, &[]).await;
    println!("Table created");
}

async fn do_things<T>(client: &T)
where
    T: Client,
{
    //let sql = "INSERT INTO test (name) VALUES ($1)";
    let sql = "INSERT INTO test (name) VALUES (?)";
    //let sql = "INSERT INTO test (name) VALUES (@P1)";
    for n in 1..10 {
        let test = format!("TEST: {}", n);
        //client.execute(sql, &[]).await.unwrap();
        client.execute(sql, &[&test]).await.unwrap();
    }

    let rows = client
        .fetch_rows("SELECT name from test", &[])
        .await
        .unwrap();

    for row in rows {
        let name: String = row.get("name").unwrap();
        println!("ROW: {:?}", &name);
    }
}
