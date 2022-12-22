use sqlx::Connection;
use sqlx::SqliteConnection;

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    let _pool = SqliteConnection::connect("sqlite::memory:").await?;

    //// Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
    //let row: (i64,) = sqlx::query_as("SELECT $1")
    //    .bind(150_i64)
    //    .fetch_one(&pool)
    //    .await?;

    //assert_eq!(row.0, 150);

    Ok(())
}
