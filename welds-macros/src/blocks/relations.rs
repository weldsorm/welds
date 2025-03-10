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

    let relation_traits: Vec<_> = relations.iter().map(|x| relationdef(info, x)).collect();
    let relation_traits = quote! { #(#relation_traits) * };
    let belongs_impl: Vec<_> = relations.iter().map(|x| belongsdef(info, x)).collect();
    let belongs_impl = quote! { #(#belongs_impl) * };

    quote! {

        #relation_traits

        #belongs_impl

        impl #wp::relations::HasRelations for #defstruct {
            type Relation = #relations_struct;

            fn relations() -> Self::Relation {
                Self::Relation::default()
            }
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

fn fielddef(info: &Info, relation: &Relation) -> TokenStream {
    let wp = &info.welds_path;
    let kind = &relation.kind;
    let field = &relation.field;
    let other = &relation.foreign_struct;
    quote! {
        pub #field: #wp::relations::#kind<#other>
    }
}

fn defaultdef(info: &Info, relation: &Relation) -> TokenStream {
    let wp = &info.welds_path;
    let kind = &relation.kind;
    let field = &relation.field;
    let fk = &relation.foreign_key;
    quote! {
        #field: #wp::relations::#kind::using(#fk)
    }
}

fn relationdef(info: &Info, relation: &Relation) -> TokenStream {
    let wp = &info.welds_path;
    let kind = &relation.kind;
    let defstruct = &info.defstruct;
    let related = &relation.foreign_struct;

    quote! { impl #wp::relations::Related<#wp::relations::#kind<#related>> for #defstruct {} }
}

fn belongsdef(info: &Info, relation: &Relation) -> TokenStream {
    let wp = &info.welds_path;
    let cols = &info.columns;
    let kind = &relation.kind;
    let fk = &relation.foreign_key;
    let defstruct = &info.defstruct;
    let other = &relation.foreign_struct;

    match cols.iter().find(|&c| c.field.to_string() == fk.to_string()) {
        Some(fk_column) => {
            let fk_name = &fk_column.field;
            let fk_type = &fk_column.field_type;

            let fk_value_type = if fk_column.is_option {
                quote! { Option<#fk_type> }
            } else {
                quote! { #fk_type }
            };

            let fk_value_inner = if fk_column.is_option {
                quote! { self.#fk_name.clone().unwrap_or_default() }
            } else {
                quote! { self.#fk_name.clone() }
            };

            match kind.to_string().as_str() {
                "BelongsTo" => {
                    return quote! {

                        impl #wp::relations::BelongsToFkValue<#other> for #defstruct {
                            type FkVal = #fk_value_type;

                            fn fk_value<R>(&self) -> Self::FkVal
                            where
                                <Self as #wp::model_traits::HasSchema>::Schema: #wp::model_traits::TableInfo + #wp::model_traits::TableColumns,
                            {
                                self.#fk_name.clone()
                            }
                        }

                    }
                }
                "HasOne" => {
                    return quote! {

                        impl #wp::relations::HasOneFkValue<#other> for #defstruct {
                            type HasOneFkVal = #fk_value_type;
                            type HasOneFkValInner = #fk_type;

                            fn fk_value<R>(&self) -> Self::HasOneFkVal
                            where
                                <Self as #wp::model_traits::HasSchema>::Schema: #wp::model_traits::TableInfo + #wp::model_traits::TableColumns,
                            {
                                self.#fk_name.clone()
                            }

                            fn fk_value_inner<R>(&self) -> Self::HasOneFkValInner
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
        None => {},
    }

    quote! {}
}
