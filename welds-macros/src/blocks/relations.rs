use crate::{info::Info, relation::Relation};
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn write(infos: &Info) -> TokenStream {
    let defstruct = &infos.defstruct;
    let relations_struct = &infos.relations_struct;
    let relations = infos.relations.as_slice();
    if relations.is_empty() {
        return quote! {};
    }

    let struct_fields: Vec<_> = relations.iter().map(fielddef).collect();
    let struct_fields = quote! { #(#struct_fields), * };
    let default_fields: Vec<_> = relations.iter().map(defaultdef).collect();
    let default_fields = quote! { #(#default_fields), * };

    quote! {

        impl welds::relations::HasRelations for #defstruct {
            type Relation = #relations_struct;
        }

        pub struct #relations_struct {
            #struct_fields
        }

        impl Default for #relations_struct {
            fn default() -> Self {
                Self {
                    #default_fields
                }
            }
        }

    }
}

fn fielddef(relation: &Relation) -> TokenStream {
    let kind = &relation.kind;
    let field = &relation.field;
    let other = &relation.foreign_struct;
    quote! {
        pub #field: welds::relations::#kind<#other>
    }
}

fn defaultdef(relation: &Relation) -> TokenStream {
    let kind = &relation.kind;
    let field = &relation.field;
    let fk = &relation.foreign_key;
    quote! {
        #field: welds::relations::#kind::using(#fk)
    }
}
