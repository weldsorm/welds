use crate::column::Column;
use crate::info::Info;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashSet;
use syn::Type;

/// Writes the impl for the trait
/// ForeignKeyPartialEq
///
pub(crate) fn write(info: &Info) -> TokenStream {
    // We need to impl ForeignKeyPartialEq for each type we want to be able to compare
    // Example:
    // impl ForeignKeyPartialEq<String> for Order
    // impl ForeignKeyPartialEq<i32> for Order
    // impl ForeignKeyPartialEq<i64> for Order
    //
    // This way ForeignKeyPartialEq will work for each individually. We don't want a generic that
    // restruct the value T for ALL type. That would mean PartialEq for need to be true for all

    // find all the FKs we need to support.
    let fk_fields: Vec<_> = info
        .relations
        .iter()
        .map(|r| r.foreign_key_db.as_str())
        .collect();

    // Map them into columns so we know the Types
    let fks: Vec<&Column> = fk_fields
        .iter()
        .filter_map(|fkname| info.columns.iter().find(|c| &c.dbname == fkname))
        .collect();

    // find all the unique types that we need to support
    let fk_types: HashSet<Type> = fks.iter().map(|x| x.full_type()).collect();

    let mut implementations: Vec<TokenStream> = Vec::default();
    for fk_type in fk_types {
        // find all the fks we are about to write an impl for
        let impl_for_fk: Vec<&Column> = fks
            .iter()
            .cloned()
            .filter(|x| x.full_type().eq(&fk_type))
            .collect();
        // writes the impl
        implementations.push(impl_for_type(fk_type, impl_for_fk, info));
    }

    quote! {
        #(#implementations)*
    }
}

/* Example Impl
impl ForeignKeyPartialEq<i32> for Order {
    fn eq(&self, foreign_key_field: &str, other: &i32) -> bool {
        match foreign_key_field {
            "product_id" => self.product_id.eq(other),
            //"product_id" => return other == self.product_id,
            _ => false,
        }
    }
}
*/

fn impl_for_type(ty: Type, fks: Vec<&Column>, info: &Info) -> TokenStream {
    let defstruct = &info.defstruct;
    let wp = &info.welds_path;
    let matchers: Vec<TokenStream> = fks.iter().cloned().map(write_matcher).collect();
    let matchers = quote! { #(#matchers)* };
    quote! {
        impl #wp::model_traits::ForeignKeyPartialEq<#ty> for #defstruct {
            fn eq(&self, foreign_key_field: &str, other: &#ty) -> bool {
                match foreign_key_field {
                    //"product_id" => self.product_id.eq(other),
                    #matchers
                    _ => false,
                }
            }
        }
    }
}

fn write_matcher(col: &Column) -> TokenStream {
    let fkname = &col.dbname;
    let field = &col.field;
    quote! { #fkname => self.#field.eq(other), }
}
