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

fn relation_value_def(info: &Info, relation: &Relation) -> TokenStream {
    let wp = &info.welds_path;
    let cols = &info.columns;
    let kind = &relation.kind;
    let fk = &relation.foreign_key_rust;
    let defstruct = &info.defstruct;
    let other = &relation.foreign_struct;

    match cols.iter().find(|&c| c.field.to_string() == fk.to_string()) {
        Some(fk_column) => {
            let fk_name = &fk_column.field;
            let fk_type = &fk_column.field_type;

            let fk_value_inner = if fk_column.is_option {
                quote! { self.#fk_name.clone().unwrap_or_default() }
            } else {
                quote! { self.#fk_name.clone() }
            };

            match kind.to_string().as_str() {
                "HasOne" | "BelongsTo" => {
                    return quote! {

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
                _ => {}
            }
        }
        None => {
            let pk_name = &info.pks[0].field;
            let pk_type = &info.pks[0].field_type;

            return quote! {

                impl #wp::relations::RelationValue<#other> for #defstruct {
                    type ValueType = #pk_type;

                    fn relation_value(&self) -> Self::ValueType
                    where
                        <Self as #wp::model_traits::HasSchema>::Schema: #wp::model_traits::TableInfo + #wp::model_traits::TableColumns,
                    {
                        self.#pk_name.clone()
                    }
                }

            };
        }
    }

    quote! {}
}
