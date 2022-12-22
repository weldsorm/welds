//use sqlx::Connection;
//use sqlx::SqliteConnection;
use clap::Parser;
use std::path::PathBuf;
use weldslib::commands::Commands::*;

#[async_std::main]
async fn main() {
    let args = weldslib::commands::Args::parse();

    let schema_path = args
        .schema_file
        .clone()
        .unwrap_or_else(|| PathBuf::from("./welds.yaml"));

    let result = match args.command {
        Update { table } => weldslib::update(schema_path, table).await,
        Generate { table } => weldslib::generate(schema_path, table).await,
    };

    if let Err(err) = result {
        eprintln!("");
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
