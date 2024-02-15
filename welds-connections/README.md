
<div align="center">
  <img src="https://raw.githubusercontent.com/weldsorm/welds/main/page/src/assets/images/banner.png"/>
  <h3>An async ORM written in rust using the sqlx and tiberius.</h3>
</div>

# Welds Connections

#### This is the common interface used by welds for all Databases.

It allows you to talk to all sqlx and tiberius over a trait in a simple common way.

## Features
- async for all. 
- Connections are pooled.
- Transactions for all. (looking at you tiberius)
- Support for multiple SQL databases (Mssql, MySql, Postgres, Sqlite)
- Written for ease of development. Simple interface

## Example

```rust

use welds_connections::{Client, TransactStart};

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {

    let url = "sqlite://./test.sqlite";
    let client = welds_connections::sqlite::get_conn(url).await?;

    let sql = "SELECT name from people where name like ?";
    let filter = "James%".to_string();
    let rows = client.fetch_rows(sql, &[&filter]).await?;

    for row in rows {
        let name: String = row.get("name").unwrap();
        println!("ROW: {:?}", &name);
    }
}

```

