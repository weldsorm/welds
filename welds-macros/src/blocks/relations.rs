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

    // panic when code is invalid to give nice errors
    compiletime_asserts(info);

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

/// check that the Relation and columns are defined correctly.
/// If not will panic to give a compile time error for the user.
fn compiletime_asserts(info: &Info) {
    let columns = &info.columns;
    let relations = info.relations.as_slice();
    let defstruct = &info.defstruct;

    for relation in relations {
        let relation_kind_name = relation.kind.to_string();
        // Compile time check that the column exists.
        if &relation_kind_name == "BelongsTo" {
            let found = columns.iter().find(|x| x.dbname == relation.foreign_key_db);
            assert!(
                found.is_some(),
                "The model {} has a BelongsTo relationships pointing to the Foreign key {}, but this column is not defined on the model",
                defstruct,
                relation.foreign_key_db
            );
        }
        // Compile time check that the pk column exists.
        if &relation_kind_name == "HasMany" {
            assert!(
                !info.pks.is_empty(),
                "The model {} has a HasMany relationships defined, but no primary_key column is defined",
                defstruct,
            );
        }
        // Compile time check that the pk column exists.
        if &relation_kind_name == "BelongsToOne" {
            assert!(
                !info.pks.is_empty(),
                "The model {} has a BelongsToOne relationships defined, but no primary_key column is defined",
                defstruct,
            );
        }
        // Compile time check that the column exists.
        if &relation_kind_name == "HasOne" {
            let found = columns.iter().find(|x| x.dbname == relation.foreign_key_db);
            assert!(
                found.is_some(),
                "The model {} has a HasOne relationships pointing to the Foreign key {}, but this column is not defined on the model",
                defstruct,
                relation.foreign_key_db
            );
        }
    }
}
