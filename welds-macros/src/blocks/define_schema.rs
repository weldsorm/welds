use crate::column::Column;
use crate::info::Info;
use proc_macro2::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;

pub(crate) fn write(info: &Info) -> TokenStream {
    let name = &info.schemastruct;

    let fields: Vec<_> = info
        .columns
        .iter()
        .filter(|x| !x.ignore)
        .map(def_field)
        .collect();

    let default_fields: Vec<_> = info
        .columns
        .iter()
        .filter(|x| !x.ignore)
        .map(default_fields)
        .collect();

    quote! {

        pub struct #name {
            #(#fields),*
        }

        impl Default for #name {
            fn default() -> Self {
                Self {
                    #(#default_fields),*
                }
            }
        }

    }
}

fn def_field(col: &Column) -> TokenStream {
    let name = &col.field;
    let type_inner = &col.field_type;
    let mut ty = quote! { #type_inner };
    if col.is_option {
        ty = quote! { welds::query::optional::Optional<#type_inner> }
    }
    let clause = get_clause(type_inner, col.is_option);
    let full_type = quote! { welds::query::clause::#clause<#ty> };
    quote! { pub #name: #full_type }
}

fn get_clause(ty: &syn::Type, nullable: bool) -> TokenStream {
    let clasename = crate::utils::get_clause(ty, nullable);
    let id = Ident::new(clasename.as_str(), Span::call_site());
    quote! { #id }
}

fn default_fields(col: &Column) -> TokenStream {
    let name = &col.field;
    let type_inner = &col.field_type;
    let clause = get_clause(type_inner, col.is_option);
    let dbname = col.dbname.as_str();
    quote! { #name: welds::query::clause::#clause::new(#dbname) }
}
