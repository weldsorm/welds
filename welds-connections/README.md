
<div align="center">
  <img src="https://raw.githubusercontent.com/weldsorm/welds/main/page/src/assets/images/banner.png"/>
  <h3>An async ORM written in rust using sqlx and tiberius.</h3>
</div>

# Welds Connections

#### This is the common interface used by welds for all Databases.

It allows you to talk to sqlx and tiberius over traits in a simple common way.

## Features
- async for all. 
- Connections are pooled.
- Transactions for all. (looking at you tiberius)
- Support for multiple SQL databases (Mssql, MySql, Postgres, Sqlite)
- Written for ease of development. Simple interface

## The Simple interface

```rust
/// The common trait for database connections and transactions.
pub trait Client {
    /// Execute a sql command. returns the number of rows that were affected
    async fn execute(&self, sql: &str, params: &[&(dyn Param + Sync)]) -> Result<ExecuteResult>;

    /// Runs SQL and returns a collection of rows from the database.
    async fn fetch_rows(&self, sql: &str, params: &[&(dyn Param + Sync)]) -> Result<Vec<Row>>;

    /// Run several `fetch_rows` command on the same connection in the connection pool
    async fn fetch_many(&self, args: &[Fetch]) -> Result<Vec<Vec<Row>>>;

    // Returns what syntax (dialect) of SQL the backend is expecting
    fn syntax(&self) -> Syntax;
}

```

Thats it.

Thats All this crate is.

You get this for:
 - MySql and its transactions
 - Postgres and its transactions
 - Sqlite and its transactions
 - Mssql and its transactions


## Transactions

You can get a transaction with the TransactStart Trait.
```rust

use welds_connections::{Client, TransactStart};
#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let url = "sqlite://./test.sqlite";
    let client = welds_connections::connect(url).await?;
    let transaction = client.begin().await?;
    transaction.rollback.await?;
}
```


## Example

```rust

use welds_connections::{Client, TransactStart};

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {

    let url = "sqlite://./test.sqlite";
    let client = welds_connections::connect(url).await?;

    let sql = "SELECT name from people where name like ?";
    let filter = "James%".to_string();
    let rows = client.fetch_rows(sql, &[&filter]).await?;

    for row in rows {
        let name: String = row.get("name").unwrap();
        println!("ROW: {:?}", &name);
    }
}

```

