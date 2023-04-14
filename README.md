
<div align="center">
  <img src="https://raw.githubusercontent.com/weldsorm/welds/main/page/src/assets/images/banner.png"/>
  <h3>An async ORM written in rust using the sqlx framework.</h3>
</div>

# Welds

#### Welds is an async ORM written in rust using the sqlx framework. 

## Features
- Async for all. 
- Support for multiple SQL databases (Mssql, MySql, Postgres, Sqlite)
- Written for ease of development. Features aren't hidden behind traits. Code should be simple to write, and simple to read.
- sqlx always available when you need to drop down to something lower level

## Example Setup

```rust
#[derive(Debug, sqlx::FromRow, WeldsModel)]
#[welds(db(Postgres))]
//#[welds(db(Postgres, Mssql, Mysql, Sqlite))]
#[welds(schema= "inventory", table = "products")]
#[welds(BelongsTo(seller, super::people::People, "seller_id"))]
pub struct Product {
    #[sqlx(rename = "product_id")]
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
  let conn: sqlx::PgPool = sqlx::PgPool::connect(&url).await.unwrap();

  let products = Product::where_col(|p| p.price.equal(3.50)).run(&conn).await?;
```

### Basic Filter Across tables 
```rust
  let conn: sqlx::PgPool = sqlx::PgPool::connect(&url).await.unwrap();

  let sellers = Product::where_col(|product| product.price.equal(3.50))
        .map_query(|product| product.seller )
        .where_col(|seller| seller.name.ilike("%Nessie%") )
        .run(&conn).await?;
```

### Create And Update
```rust
  let conn: sqlx::PgPool = sqlx::PgPool::connect(&url).await.unwrap();
  
  let mut cookies = Product::new();
  cookies.name = "cookies".to_owned();
  // Creates the product cookie
  cookies.save.await(&conn)?; 
  cookies.description = "Yum".to_owned();
  // Updates the Cookies
  cookies.save.await(&conn)?; 
```

For more good examples [check out the examples](https://github.com/weldsorm/welds/tree/main/welds/examples).



