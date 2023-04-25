use anyhow::Result;
use clap::Parser;
use std::env;
use std::path::PathBuf;
use weldscli_lib::{commands::Commands::*, GenerateOption};

#[async_std::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    let args = weldscli_lib::commands::Args::parse();

    if let Some(uri) = args.database_url {
        env::set_var("DATABASE_URL", uri);
    }

    let mut schema_path = args
        .schema_file
        .clone()
        .unwrap_or_else(|| PathBuf::from("./welds.yaml"));

    if schema_path.is_dir() {
        schema_path.push("welds.yaml")
    }

    let project_dir = args
        .project_dir
        .clone()
        .unwrap_or_else(|| schema_path.parent().unwrap().to_path_buf());

    let result = match args.command {
        Update { table } => weldscli_lib::update(schema_path, table).await,
        Generate { table } => {
            let opt = GenerateOption {
                add_unknown_types: args.unknown_types,
                schema_path,
                output_path: project_dir,
                table,
            };
            weldscli_lib::generate(opt)
        }
        TestConnection => weldscli_lib::test_connection().await,
    };

    if let Err(err) = result {
        eprintln!();
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
    Ok(())
}
