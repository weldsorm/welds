use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use weldslib::{commands::Commands::*, GenerateOption};

#[async_std::main]
async fn main() -> Result<()> {
    let args = weldslib::commands::Args::parse();

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
        Update { table } => weldslib::update(schema_path, table).await,
        Generate { table } => {
            let opt = GenerateOption {
                schema_path,
                project_dir,
                table,
                ..Default::default()
            };
            weldslib::generate(opt)
        }
    };

    if let Err(err) = result {
        eprintln!("");
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
    Ok(())
}
