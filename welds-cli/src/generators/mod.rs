use crate::schema::Column;
use quote::__private::TokenStream;

pub(crate) mod models;

pub(crate) fn type_mapper(col: &Column) -> Option<TokenStream> {
    use quote::quote;
    let root_base_type = match col.r#type.to_lowercase().as_str() {
        "boolean" => quote::format_ident!("bool"),
        "integer" => quote::format_ident!("i32"),
        "bigint" => quote::format_ident!("i64"),
        "int8" => quote::format_ident!("i64"),
        "real" => quote::format_ident!("f64"),
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
