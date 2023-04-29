use super::WeldsModel;

/// The Product Table.
///
/// Start off a query on this table using. [all()](#method.all) or [where_col()](#method.where_col).
///
#[derive(Debug, sqlx::FromRow, WeldsModel)]
#[welds_path(crate)]
#[welds(db(Postgres, Mssql, Mysql, Sqlite))]
#[welds(schema = "examples", table = "products")]
#[welds(HasMany(product_orders, super::example_objects::ProductOrders, "product_id"))]
pub struct Product {
    #[welds(primary_key)]
    #[sqlx(rename = "ID")]
    pub id: i32,
    pub active: Option<bool>,
    pub description: Option<String>,
    pub name: String,
}

/// The Order Table.
///
/// Start off a query on this table using. [all()](#method.all) or [where_col()](#method.where_col).
///
/// ```
/// let q = Order::where_col(|o| o.id.equal(3) )
///     .map_query(|o| o.product_orders )
///     .map_query(|po| po.products )
///     .where_col(|p| p.active.equal(true) )
/// let count_product = q.count(&conn).await?
/// let first_product = q.limit(1).run(&conn).await?
/// ```
///
#[derive(Debug, sqlx::FromRow, WeldsModel)]
#[welds_path(crate)]
#[welds(db(Postgres, Mssql, Mysql, Sqlite))]
#[welds(schema = "examples", table = "orders")]
#[welds(HasMany(product_orders, super::example_objects::ProductOrders, "order_id"))]
pub struct Order {
    #[welds(primary_key)]
    #[sqlx(rename = "ID")]
    pub id: i32,
    pub customer: String,
    pub sell_price: i32,
}

/// The ProductOrder Join Table.
///
/// Start off a query on this table using. [all()](#method.all) or [where_col()](#method.where_col).
///
#[derive(Debug, sqlx::FromRow, WeldsModel)]
#[welds_path(crate)]
#[welds(db(Postgres, Mssql, Mysql, Sqlite))]
#[welds(schema = "examples", table = "products_orders")]
#[welds(BelongsTo(order, super::example_objects::Order, "order_id"))]
#[welds(BelongsTo(product, super::example_objects::Product, "product_id"))]
pub struct ProductOrders {
    #[welds(primary_key)]
    id: i32,
    order_id: i32,
    product_id: i32,
}
