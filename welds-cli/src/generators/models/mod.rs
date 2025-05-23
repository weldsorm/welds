use crate::GenerateOption;
use crate::config::{Config, Table};
use crate::errors::Result;
use proc_macro2::{Ident, Span};
use quote::quote;
use rust_format::{Formatter, RustFmt};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
mod struct_def;

pub(crate) const GENERATED_WARNING: &str = "
/******************************************************************************
 * This file was auto-generated by welds-cli.
 * changes to this file will be overridden when the welds-cli generate command runs again.
 *
 * If you want to take control of this file and prevent welds-cli from overriding it,
 * You can set the flag `manual_update: true` for this table in the welds.yaml config.
******************************************************************************/
";

pub fn run(config: &Config, opt: &GenerateOption) -> Result<()> {
    let all = &config.tables;
    let mut generated = Vec::default();

    for table in all {
        if opt.table.is_none() || opt.table == Some(table.name.to_string()) {
            let path = model_path(&opt.output_path, table);
            fs::create_dir_all(&path)?;
            init_table_mod_file(&path)?;
            struct_def::generate(&path, table, all.as_slice(), opt.hide_unknown_types)?;
            generated.push(table);
        }
    }
    init_models_mod_file(&opt.output_path, &generated)?;
    Ok(())
}

fn init_models_mod_file(path: &PathBuf, tables: &[&Table]) -> Result<()> {
    let mut path = PathBuf::from(path);
    path.push("mod.rs");
    if path.exists() {
        return Ok(());
    }
    let mut parts = Vec::default();
    for table in tables {
        let modulename = Ident::new(&table.module_name(), Span::call_site());
        parts.push(quote! { pub mod #modulename; });
    }
    let code = quote! { #(#parts)* };
    let mut file = File::create(path)?;

    let formated = RustFmt::default().format_str(code.to_string()).unwrap();
    file.write_all(formated.as_bytes())?;
    Ok(())
}

fn init_table_mod_file(path: &PathBuf) -> Result<()> {
    let mut path = PathBuf::from(path);
    path.push("mod.rs");
    if path.exists() {
        return Ok(());
    }

    let code = quote! {
        mod definition;
        pub use definition::*;
    };

    let mut file = File::create(path)?;
    let formated = RustFmt::default().format_str(code.to_string()).unwrap();
    file.write_all(formated.as_bytes())?;
    Ok(())
}

fn model_path(start_dir: &PathBuf, table: &Table) -> PathBuf {
    let mut path = PathBuf::from(start_dir);
    path.push(table.module_name());
    path
}
