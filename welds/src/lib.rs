//!
//! <div align="center">
//!   <img src="https://raw.githubusercontent.com/weldsorm/welds/main/page/src/assets/images/banner.png"/>
//!   <h3>An async ORM written in rust using sqlx and/or Tiberius.</h3>
//! </div>
//!
//! # What is Welds
//! Welds is an ORM for the Rust programming language.
//! The primary way to interact with Welds is by adding the "welds" macros to your structs.
//! This adds to your structs Welds functions and allows for great autocompletion and very intuitive code.
//!
//! # Macros - Setup your structs
//!
//! To add welds to your struct you will need to derive `welds::WeldsModel`
//! ```rust,ignore
//! #[derive(welds::WeldsModel)]
//! ```
//!
//! ## Connect to table / view
//! Let welds know what DB table/schema it will be reading/writing data from
//! ```rust,ignore
//! #[welds(schema = "public", table = "products")]
//! ```
//! if you don't want to allow writing to the table you can add readonly. (useful for views)
//! ```rust,ignore
//! #[welds(readonly)]
//! ```
//!
//! ## Build Relations for Joining
//! You can write queries that join across tables if you Wireup welds with relationships
//!
//! Welds Supports:
//! - BelongsTo
//! - HasOne
//! - HasMany
//!
//! They are both in the format:
//! ```rust,ignore
//! [field, rust_path_to_other_object, "str_of_foreign_key_column"]
//! ```
//! NOTE: Both `BelongsTo` and `HasMany` need to know the foreign_key column
//!
//! If you are working on a struct that has a foreign_key to another table use a `BelongsTo`
//!
//! ```rust,ignore
//! #[welds(BelongsTo(product, super::product::product_id, "product_id"))]
//! struct Order {
//!     product_id: ...,
//!     ...,
//! }
//! ```
//!
//! If you are working on a struct that has a foreign_key pointed at it, use a `HasMany`
//! ```rust,ignore
//! #[welds(HasMany(orders, super::order::Order, "product_id"))]
//! struct Product {
//!     ...,
//! }
//! ```
//!
//! If working with a many-to-many relationship, add a `HasMany` on both structs, and add two `BelongsTo`
//! on the join table struct
//! ```rust,ignore
//! #[welds(BelongsTo(order, super::order::Order, "order_id"))]
//! #[welds(BelongsTo(product, super::product::Product, "product_id"))]
//! struct ProductOrders {
//!   #[welds(primary_key)]
//!   id: i32,
//!   order_id: i32,
//!   product_id: i32,
//! }
//! ```
//!
//! ## Fields Level Attributes
//! There are a couple attributes you can add to your fields to help control how welds functions
//! - `#[welds(primary_key)]` Important! Add this to the primary key of your table.
//! - `#[welds(rename = "xyz")]` let welds know the underlying column has a different name than the field
//! - `#[welds(ignore)]` Tell welds this field is it not in the database.
//!
//!
//! ## Putting it all together
//! Here is a working example of what a fully setup struct might look like
//! ```rust,ignore
//! #[derive(Debug, welds::WeldsModel)]
//! #[welds(schema = "public", table = "products")]
//! #[welds(HasMany(order, super::order::Order, "product_id"))]
//! pub struct Product {
//!     #[welds(primary_key)]
//!     #[welds(rename = "product_id")]
//!     pub id: i32,
//!     pub active: Option<bool>,
//!     pub description: Option<String>,
//!     pub name: String,
//! }
//! ```
//!
//! # Using Welds
//!
//! When WeldsModel is added to your struct the following is added directly to your model
//!
//! - [`Model::new()`]( ./state/struct.DbState.html#method.new_uncreated ) make a new model ready to be saved to the database
//! - `Model::find_by_id()`
//! - [`Model::all()`](./query/builder/struct.QueryBuilder.html) start a query for a Model
//! - [`Model::where_col()`](./query/builder/struct.QueryBuilder.html) start a query for a Model
//! - `Model::from_raw_sql()` finds Model using raw custom SQL str
//!
//! Instances of your model are wrapped in a [welds::state::DbState](./state/struct.DbState.html).
//! From your instance you can update, create, and delete.
//!
//! Start a query from the struct that derived WeldsModel
//! ```rust,ignore
//! let client = welds::connections::postgres::connect(&url).await.unwrap();
//! let sellers = Product::all()
//!       .where_col(|product| product.price.equal(3.50))
//!       .map_query(|product| product.seller )
//!       .where_col(|seller| seller.name.ilike("%Nessie%") )
//!       .order_by_desc(|seller| seller.id )
//!       .limit( 10 )
//!       .run(&client).await?;
//!
//! ```
//!
//! For more examples on how to use Welds check out the [Example Repo](https://github.com/weldsorm/welds/tree/main/welds/examples)
//!
//! ## Examples:
//!  - [Basic CRUD](https://github.com/weldsorm/welds/blob/main/welds/examples/crud.rs)
//!  - [Mapping Queries / Joining](https://github.com/weldsorm/welds/blob/main/welds/examples/crud.rs)
//!  - [Bulk (Create/Update/Delete)](https://github.com/weldsorm/welds/blob/main/welds/examples/bulk_operations.rs)
//!  - [Select Only Specific Columns](https://github.com/weldsorm/welds/blob/main/welds/examples/manual_select_columns.rs)
//!  - [Hooks, Callback when models (Save/Update/Delete)](https://github.com/weldsorm/welds/blob/main/welds/examples/hooks.rs)
//!  - [Scopes for your Models](https://github.com/weldsorm/welds/blob/main/welds/examples/scopes.rs)
//!  - [Migrations](https://github.com/weldsorm/welds/blob/main/welds/examples/migrations.rs)
//!  - [Checking DB schema matches compiled structs](https://github.com/weldsorm/welds/blob/main/welds/examples/verify_tables.rs)
//
//!
//! # Features
//!
//! - postgres - enables postgres database connection. (requires sqlx setup)
//! - mysql - enables MySql database connection. (requires sqlx setup)
//! - sqlite - enables Sqlite database connection. (requires sqlx setup)
//! - mssql - enables Microsoft SQL support. (requires tokio runtime.)
//! - detect - enables scanning of the database to get schema info
//! - check - enables checking your models against table in the database
//! - migrations - adds all the migration structs and traits
//! - full - all the features excluding (mock)
//! - mock - Use for testing ONLY. Enables mocking out database schemas
//!
//!
//! # Important Notes:
//!
//! - If you are using one of the `sqlx` connections, you will need to setup sqlx. This is so you can pick an async runtime.
//!

pub mod errors;
pub use errors::WeldsError;
pub mod dataset;
pub mod exts;
pub mod model_traits;
pub mod query;
pub mod relations;
pub mod state;
pub mod writers;

pub mod prelude;

#[cfg(feature = "detect")]
pub mod detect;

#[cfg(feature = "check")]
pub mod check;

#[cfg(feature = "migrations")]
pub mod migrations;

pub use welds_connections as connections;

/// Re-export welds_connections
pub use welds_connections::{Client, Row, Syntax, TransactStart};

/// Re-export the Macro used to make models
pub use welds_macros::WeldsModel;
