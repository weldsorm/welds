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

    // we define a impl RelationValue<_> for each Relationship defined on the struct
    // [#welds(HasMany, .., ..)]
    let relation_value_impl: Vec<_> = relations
        .iter()
        .map(|x| relation_value_def(info, x))
        .collect();
    let relation_value_impl = quote! { #(#relation_value_impl) * };

    quote! {

        #relation_value_impl

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
            fn check<R>(&self, other: &R) -> bool
            where
                R: #wp::relations::RelationValue<Self>,
                Self: #wp::relations::RelationValue<R>,
            {
                let self_value = <Self as #wp::relations::RelationValue<R>>::relation_value(self);
                let other_value = <R as #wp::relations::RelationValue<Self>>::relation_value(other);

                if let Some(downcast_other_value) = (&other_value as &dyn std::any::Any).downcast_ref::<<Self as #wp::relations::RelationValue<R>>::ValueType>() {
                    return &self_value == downcast_other_value
                }

                false
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

// writes impl RelationValue for a given Relation
fn relation_value_def(info: &Info, relation: &Relation) -> TokenStream {
    let kind = &relation.kind;

    // first we need to see if the FK is on this table other the other table.
    // That way we know where to get the value we are looking for
    let kind_name = kind.to_string();
    let fk_on_other = &kind_name == "HasOne" || &kind_name == "BelongsTo";

    if fk_on_other {
        relation_value_def_from_fk(info, relation)
    } else {
        relation_value_def_from_pk(info, relation)
    }
}

// writes impl RelationValue for a given Relation
// Reading the value from a foreign_key field
fn relation_value_def_from_fk(info: &Info, relation: &Relation) -> TokenStream {
    let wp = &info.welds_path;
    let cols = &info.columns;
    //let kind = &relation.kind;
    let defstruct = &info.defstruct;
    let other = &relation.foreign_struct;
    let fk = &relation.foreign_key_rust;

    let fk_col = cols.iter().find(|x| x.field == fk);
    // If the fk column is missing, panic to give a compile time error.
    // The user has defined a relationship that points to a rust field that doesn't exist.
    let fk_column = fk_col.unwrap_or_else(|| panic!("The model {} is missing the field {}. This field was expected because it was defined in a Welds Relationship", defstruct, fk));

    // build the inner content that reads the field.
    let fk_name = &fk_column.field;
    let fk_type = &fk_column.field_type;
    let fk_value_inner = if fk_column.is_option {
        quote! { self.#fk_name.clone().unwrap_or_default() }
    } else {
        quote! { self.#fk_name.clone() }
    };

    quote! {
        impl #wp::relations::RelationValue<#other> for #defstruct {
            type ValueType = #fk_type;
            fn relation_value(&self) -> Self::ValueType
            where
                <Self as #wp::model_traits::HasSchema>::Schema: #wp::model_traits::TableInfo + #wp::model_traits::TableColumns,
            {
                #fk_value_inner
            }
        }
    }
}

// writes impl RelationValue for a given Relation
// Reading the value from the primary_key field
fn relation_value_def_from_pk(info: &Info, relation: &Relation) -> TokenStream {
    let wp = &info.welds_path;
    let other = &relation.foreign_struct;
    let defstruct = &info.defstruct;
    // If the model doesn't have a primary_key, panic to give the user a compile time error
    // The user has defined a relationship that points to a rust field that doesn't exist.
    let pk_column = info.pks.first();
    let pk_column = pk_column.unwrap_or_else(|| panic!("The model {} is missing a primary key. This field was expected because it was defined in a Welds Relationship", defstruct));

    let pk_name = &pk_column.field;
    let pk_type = &pk_column.field_type;

    quote! {

        impl #wp::relations::RelationValue<#other> for #defstruct {
            type ValueType = #pk_type;
            fn relation_value(&self) -> Self::ValueType
            where
                <Self as #wp::model_traits::HasSchema>::Schema: #wp::model_traits::TableInfo + #wp::model_traits::TableColumns,
            {
                self.#pk_name.clone()
            }
        }

    }
}
