use clap::{Parser, Subcommand, ValueEnum};
use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Parser)] // requires `derive` feature
#[command(name = "welds")]
#[command(about = "A post-modern ORM", long_about = None)]
pub(crate) struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    /// Update the knowledge of database structures
    Update { table: Option<String> },
    /// Generate new models in your code based on the knowledge of the database
    Generate { table: Option<String> },
}
