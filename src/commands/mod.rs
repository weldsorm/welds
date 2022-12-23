use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)] // requires `derive` feature
#[command(name = "welds")]
#[command(about = "A post-modern ORM", long_about = None)]
pub struct Args {
    /// Set the path to the schema definition file (defaults to the current directory)
    #[arg(short, long, value_name = "schema")]
    pub schema_file: Option<PathBuf>,
    /// Set the path to the root of your project (defaults to the path of the schema file)
    #[arg(short, long, value_name = "project")]
    pub project_dir: Option<PathBuf>,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Update the knowledge of database structures
    Update { table: Option<String> },
    /// Generate new models in your code based on the knowledge of the database
    Generate { table: Option<String> },
}
