use clap::{Parser, Subcommand, ValueEnum};
use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Parser)] // requires `derive` feature
#[command(name = "welds")]
#[command(about = "A post-modern ORM", long_about = None)]
pub struct Args {
    /// Set the path to the schema definition file
    #[arg(short, long, value_name = "schema")]
    pub schema_file: Option<PathBuf>,
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
