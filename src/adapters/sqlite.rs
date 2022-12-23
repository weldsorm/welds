use sqlx::sqlite::SqlitePool;
use std::env;
use anyhow::Result;

pub async fn schema() -> Result<()> {
    let uri = env::var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&uri).await?;


    // Insert the task, then obtain the ID of this row
    let results = sqlx::query!(r#"SELECT * FROM sqlite_master AS m "#).fetch_all(&pool).await?;

    println!("Results: {:?}", results);

    Ok(())
}
