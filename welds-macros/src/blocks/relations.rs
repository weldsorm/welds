use crate::{info::Info, relation::Relation};
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn write(info: &Info) -> TokenStream {
    let wp = &info.welds_path;
    let defstruct = &info.defstruct;
    let relations_struct = &info.relations_struct;
    let relations = info.relations.as_slice();
    if relations.is_empty() {
        return quote! {};
    }

    let struct_fields: Vec<_> = relations.iter().map(|x| fielddef(info, x)).collect();
    let struct_fields = quote! { #(#struct_fields), * };
    let default_fields: Vec<_> = relations.iter().map(|x| defaultdef(info, x)).collect();
    let default_fields = quote! { #(#default_fields), * };

    quote! {

        // build the HasRelations struct used for lambda selection of relationships
        impl #wp::relations::HasRelations for #defstruct {
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

        impl #wp::model_traits::CheckRelationship for #defstruct {
            fn check<R, Ship>(&self, other: &R, relations: &Ship) -> bool
            where
                Ship: #wp::relations::Relationship<R> {
                    todo!()
            }
        }

    }
}

// write the definition of a HasRelations field
fn fielddef(info: &Info, relation: &Relation) -> TokenStream {
    let wp = &info.welds_path;
    let kind = &relation.kind;
    let field = &relation.field;
    let other = &relation.foreign_struct;
    quote! {
        pub #field: #wp::relations::#kind<#other>
    }
}

// write the default assignment for the HasRelations field
fn defaultdef(info: &Info, relation: &Relation) -> TokenStream {
    let wp = &info.welds_path;
    let kind = &relation.kind;
    let field = &relation.field;
    let fk = &relation.foreign_key_db;
    quote! {
        #field: #wp::relations::#kind::using(#fk)
    }
}
