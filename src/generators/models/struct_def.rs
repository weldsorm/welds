use crate::errors::Result;
use crate::generators::type_mapper;
use crate::schema::Table;
use quote::{format_ident, quote};
use rust_format::{Formatter, RustFmt};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

pub(crate) fn generate(mod_path: &PathBuf, table: &Table) -> Result<()> {
    let mut path = PathBuf::from(mod_path);
    path.push("definition.rs");

    let struct_name = format_ident!("{}", table.struct_name());

    let mut fields = Vec::default();
    for col in &table.schema {
        if let Some(tt) = type_mapper(col) {
            let name = format_ident!("{}", col.name);
            let feild = quote! { #name: #tt };
            fields.push(feild);
        }
    }
    let fields = quote! { #(#fields),* };

    let code = quote! {
        struct #struct_name {
            #fields
        }
    };

    let mut file = File::create(path)?;
    let formated = RustFmt::default().format_str(code.to_string()).unwrap();
    file.write_all(formated.as_bytes())?;
    Ok(())
}
