
<div align="center">
  <img src="https://raw.githubusercontent.com/weldsorm/welds/main/page/src/assets/images/banner.png"/>
  <h3>An async ORM written in rust using sqlx and/or Tiberius.</h3>
</div>



# Welds

#### Welds is an async ORM written in rust using sqlx and/or Tiberius. 

## Features
- Async for all. 
- Support for multiple SQL databases (Mssql, MySql, Postgres, Sqlite)
- Written for ease of development. Features aren't hidden behind traits. Code should be simple to write, and simple to read.
- Low level connection always available when you need to drop down to raw SQL.

Under the hood welds uses:
- sqlx for Postgres, MySql, and Sqlite.
- Tiberius for MSSQL

Compatibility:
- the `0.4.*` line of welds is compiled with sqlx 0.8
- the `0.3.*` line of welds is compiled with sqlx 0.7

## Example Setup

```rust
#[derive(Debug, WeldsModel)]
#[welds(schema= "inventory", table = "products")]
#[welds(BelongsTo(seller, super::people::People, "seller_id"))]
pub struct Product {
    #[welds(rename = "product_id")]
    #[welds(primary_key)]
    pub id: i32,
    pub name: String,
    pub seller_id: Option<i32>,
    pub description: Option<String>,
    pub price: Option<f32>,
}
```


## Example Usage

### Basic Select 
```rust
  let url = "postgres://postgres:password@localhost:5432";
  let client = welds::connections::postgres::connect(url).await.unwrap();

  let products = Product::where_col(|p| p.price.equal(3.50)).run(&client).await?;
```

### Basic Filter Across tables
```rust
  let client = welds::connections::mssql::connect(url).await.unwrap();

  let sellers = Product::where_col(|product| product.price.equal(3.50))
        .map_query(|product| product.seller )
        .where_col(|seller| seller.name.ilike("%Nessie%") )
        .run(&client).await?;
```

### Create And Update
```rust
  let client = welds::connections::sqlite::connect(url).await.unwrap();

  let mut cookies = Product::new();
  cookies.name = "cookies".to_owned();
  // Creates the product cookie
  cookies.save(&client).await?;
  cookies.description = "Yum".to_owned();
  // Updates the Cookies
  cookies.save(&client).await?;
```


### Types from external crates

Both `Tiberius` and `sqlx` support types from external crates such at `chrono` and `serde_json`. These types need to be enabled in the underlying crate to use.
In order to use types that are external the appropriate feature needs to be enabled in these underlying frameworks.
We have chosen to leave this up to you as the developer so you have full control over your underlying SQLX/Tiberius setup.

In order to get these types to work you will need to:
1) Add the external crate `cargo add chrono`
2) Enable the feature in the underlying SQL framework. `cargo add sqlx --features=chrono`
3) (Tiberius only) enable the corresponding feature for welds-connections feature `cargo add welds-connections --features=mssql,mssql-chrono`

welds-connections features needed for mssql (tiberius):
* mssql-chrono
* mssql-time
* mssql-rust_decimal
* mssql-bigdecimal


## Other Examples
 - [Basic CRUD](https://github.com/weldsorm/welds/blob/main/welds/examples/crud.rs)
 - [Mapping Queries / Joining](https://github.com/weldsorm/welds/blob/main/welds/examples/crud.rs)
 - [Bulk (Create/Update/Delete)](https://github.com/weldsorm/welds/blob/main/welds/examples/bulk_operations.rs)
 - [Select Only Specific Columns](https://github.com/weldsorm/welds/blob/main/welds/examples/manual_select_columns.rs)
 - [Fetch related objects with include](https://github.com/weldsorm/welds/blob/main/welds/examples/includes.rs)
 - [Hooks, Callback when models (Save/Update/Delete)](https://github.com/weldsorm/welds/blob/main/welds/examples/hooks.rs)
 - [Scopes for your Models](https://github.com/weldsorm/welds/blob/main/welds/examples/scopes.rs)
 - [Migrations](https://github.com/weldsorm/welds/blob/main/welds/examples/migrations.rs)
 - [Wrapping operations in Transactions](https://github.com/weldsorm/welds/blob/main/welds/examples/transactions.rs)
 - [Checking DB schema matches compiled structs](https://github.com/weldsorm/welds/blob/main/welds/examples/verify_tables.rs)

For more good examples [check out the examples repo](https://github.com/weldsorm/welds/tree/main/welds/examples).
