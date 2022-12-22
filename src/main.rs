use sqlx::Connection;
use sqlx::SqliteConnection;
mod commands;
use clap::Parser;

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    let args = commands::Args::parse();

    //let _pool = SqliteConnection::connect("sqlite::memory:").await?;
    Ok(())
}
