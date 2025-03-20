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

    // build a list of all external types we want to be able to compare to
    let fk_types: HashSet<Type> = fks.iter().map(|x| x.field_type.clone()).collect();

    let mut implementations: Vec<TokenStream> = Vec::default();
    for fk_type in fk_types {
        // find all the fks Columns that will be in the match for this type.
        let impl_for_fk: Vec<&Column> = fks
            .iter()
            .cloned()
            .filter(|x| x.field_type.eq(&fk_type))
            .collect();

        // writes the impl for basic and optional variants of the type
        implementations.push(impl_for_type_non_opt(fk_type.clone(), &impl_for_fk, info));
        implementations.push(impl_for_type_opt(fk_type.clone(), &impl_for_fk, info));
    }

    quote! {
        #(#implementations)*
    }
}

fn impl_for_type_opt(other_ty: Type, fks: &[&Column], info: &Info) -> TokenStream {
    let defstruct = &info.defstruct;
    let wp = &info.welds_path;
    let matchers: Vec<TokenStream> = fks.iter().cloned().map(write_option_matcher).collect();
    let matchers = quote! { #(#matchers)* };
    quote! {
        impl #wp::model_traits::ForeignKeyPartialEq<Option<#other_ty>> for #defstruct {
            fn eq(&self, foreign_key_field: &str, other: &Option<#other_ty>) -> bool {
                match foreign_key_field {
                    #matchers
                    _ => false,
                }
            }
        }
    }
}

fn impl_for_type_non_opt(other_ty: Type, fks: &[&Column], info: &Info) -> TokenStream {
    let defstruct = &info.defstruct;
    let wp = &info.welds_path;
    let matchers: Vec<TokenStream> = fks.iter().cloned().map(write_basic_matcher).collect();
    let matchers = quote! { #(#matchers)* };
    quote! {
        impl #wp::model_traits::ForeignKeyPartialEq<#other_ty> for #defstruct {
            fn eq(&self, foreign_key_field: &str, other: &#other_ty) -> bool {
                match foreign_key_field {
                    #matchers
                    _ => false,
                }
            }
        }
    }
}

fn write_basic_matcher(col: &Column) -> TokenStream {
    // Other is NOT option.
    // self.field COULD be option
    let fkname = &col.dbname;
    let field = &col.field;
    if col.is_option {
        quote! { #fkname => if self.#field.is_none() { false } else { self.#field.as_ref().unwrap().eq(other) }  }
    } else {
        quote! { #fkname => self.#field.eq(other), }
    }
}

fn write_option_matcher(col: &Column) -> TokenStream {
    // Other IS option
    // self.field COULD be option
    let fkname = &col.dbname;
    let field = &col.field;
    if col.is_option {
        quote! { #fkname => self.#field.eq(other), }
    } else {
        quote! { #fkname => if other.is_none() { false } else { self.#field.eq(other.as_ref().unwrap()) }  }
    }
}
