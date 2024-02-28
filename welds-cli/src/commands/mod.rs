use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)] // requires `derive` feature
#[command(name = "welds")]
#[command(about = "A post-modern ORM", long_about = None)]
pub struct Args {
    /// Set the path to the schema definition file (defaults to the current directory)
    #[arg(short, long, value_name = "schema")]
    pub schema_file: Option<PathBuf>,
    /// Set the path to where the generated models will be saved (defaults to the path of the schema file)
    #[arg(short, long, value_name = "project")]
    pub project_dir: Option<PathBuf>,

    /// Set the DATABASE_URL which will be used in the connection
    #[arg(short, long, value_name = "database_url")]
    pub database_url: Option<String>,

    #[command(subcommand)]
    pub command: Commands,

    /// Force add unknown types to be hidden when generating models.
    /// NOTE: you will need to resolve compile errors for unknown types.
    #[arg(short, long, value_name = "hide_unknown_types")]
    pub hide_unknown_types: bool,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Update the knowledge of database structures
    Update { table: Option<String> },
    /// Generate new models in your code based on the knowledge of the database
    Generate { table: Option<String> },
    /// Verify Welds can connect to the database in DATABASE_URL
    TestConnection,
}
