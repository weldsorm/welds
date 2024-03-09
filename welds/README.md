
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
  cookies.save.await(&client)?; 
  cookies.description = "Yum".to_owned();
  // Updates the Cookies
  cookies.save.await(&client)?; 
```

## Other Examples
 - [Basic CRUD](https://github.com/weldsorm/welds/blob/main/welds/examples/crud.rs)
 - [Mapping Queries / Joining](https://github.com/weldsorm/welds/blob/main/welds/examples/crud.rs)
 - [Bulk (Create/Update/Delete)](https://github.com/weldsorm/welds/blob/main/welds/examples/bulk_operations.rs)
 - [Select Only Specific Columns](https://github.com/weldsorm/welds/blob/main/welds/examples/manual_select_columns.rs)
 - [Checking DB schema matches compiled structs](https://github.com/weldsorm/welds/blob/main/welds/examples/verify_tables.rs)

For more good examples [check out the examples repo](https://github.com/weldsorm/welds/tree/main/welds/examples).



