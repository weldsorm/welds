use crate::errors::Result;
use crate::schema::{Config, Table};
use crate::GenerateOption;
use rust_format::{Formatter, RustFmt};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

pub fn run(config: &Config, opt: &GenerateOption) -> Result<()> {
    //match sure we are working in a valid project
    super::validate_project_path(&opt.project_dir)?;

    let tables: Vec<_> = config
        .tables
        .iter()
        .filter(|x| opt.table.is_none() || opt.table == Some(x.name.to_string()))
        .collect();

    for table in tables {
        let path = model_path(&opt.project_dir, &table);
        fs::create_dir_all(&path)?;
        init_files(&path)?;
    }

    Ok(())
}

fn init_files(path: &PathBuf) -> Result<()> {
    let mut path = PathBuf::from(path);
    path.push("mod.rs");

    let code = quote::quote! {
        mod definition;
        mod genearted;
        mod customizations;
        pub use definition::*;
        pub use genearted::*;
        pub use customizations::*;
    };
    let mut file = File::create(path)?;
    let formated = RustFmt::default().format_str(code.to_string()).unwrap();
    file.write_all(formated.as_bytes())?;
    Ok(())
}

fn model_path(project_dir: &PathBuf, table: &Table) -> PathBuf {
    let mut path = PathBuf::from(project_dir);
    path.push("src");
    path.push("models");
    path.push(table.module_name());
    path
}
