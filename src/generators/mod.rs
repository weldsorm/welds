use quote::__private::TokenStream;

use crate::{
    errors::{Result, WeldsError},
    schema::Schema,
};
use std::path::PathBuf;

pub(crate) mod models;

fn validate_project_path(path: &PathBuf) -> Result<()> {
    if !path.exists() || !path.is_dir() {
        return Err(WeldsError::InvalidProject);
    }

    let mut src = PathBuf::from(path);
    src.push("src");
    if !src.exists() || !src.is_dir() {
        return Err(WeldsError::InvalidProject);
    }

    let mut cargo_toml = PathBuf::from(path);
    cargo_toml.push("Cargo.toml");
    if !cargo_toml.exists() || !cargo_toml.is_file() {
        return Err(WeldsError::InvalidProject);
    }

    // we could run cargo check...

    Ok(())
}

pub(crate) fn type_mapper(col: &Schema) -> Option<TokenStream> {
    use quote::quote;
    let root_base_type = match col.r#type.as_str() {
        "integer" => quote::format_ident!("u32"),
        "text" => quote::format_ident!("String"),
        "blob" => quote::format_ident!("u8"),
        _ => return None,
    };
    let mut q = quote! { #root_base_type };
    let is_vec = match col.r#type.as_str() {
        "blob" => true,
        _ => false,
    };
    if is_vec {
        q = quote! { Vec<#q>};
    }
    if col.null {
        q = quote! { Option<#q>};
    }
    Some(q)
}
