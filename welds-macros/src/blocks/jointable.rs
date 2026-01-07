use crate::{info::Info, relation::Relation};
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn write(info: &Info) -> TokenStream {
    let join_relations: Vec<_> = info.relations.iter().filter(|r| r.is_jointable).collect();

    if join_relations.len() != 2 {
        return quote!();
    }

    // unwrap to get the two sides of joins we are working with
    let (join_a, join_b) = (join_relations[0], join_relations[1]);

    let p1 = impl_has_fk(info, join_a);
    let p2 = impl_has_fk(info, join_b);
    let p3 = write_link_unlink(info, join_a, join_b);

    quote!( #p1 #p2 #p3 )
}

fn impl_has_fk(info: &Info, relation: &Relation) -> TokenStream {
    let wp = &info.welds_path;
    let model_struct = &info.defstruct;
    let foreign_struct = &relation.foreign_struct;
    let foreign_key = &relation.foreign_key_db;
    quote! {
        impl #wp::relations::HasJoinTableForeignkey<#foreign_struct> for #model_struct {
            fn fk_column() -> &'static str {
                #foreign_key
            }
        }
    }
}

fn write_link_unlink(info: &Info, relation_a: &Relation, relation_b: &Relation) -> TokenStream {
    let wp = &info.welds_path;
    let model_struct = &info.defstruct;
    let struct_a = &relation_a.foreign_struct;
    let struct_b = &relation_b.foreign_struct;
    let async_token = if cfg!(feature = "__sync") {
        quote! {}
    } else {
        quote! { async }
    };
    let await_token = if cfg!(feature = "__sync") {
        quote! {}
    } else {
        quote! { .await }
    };
    quote! {

        impl #model_struct {
            pub #async_token fn link(
                model_a: &mut #wp::state::DbState<#struct_a>,
                model_b: &mut #wp::state::DbState<#struct_b>,
                client: &dyn Client,
            ) -> welds::errors::Result<()> {
                // save if needed
                if model_a.db_status() != #wp::state::DbStatus::NotModified {
                    model_a.save(client)#await_token?;
                }
                if model_b.db_status() != #wp::state::DbStatus::NotModified {
                    model_b.save(client)#await_token?;
                }
                let a: &#struct_a = model_a;
                let b: &#struct_b = model_b;
                #wp::query::link::create::<#model_struct, _, _>(client, a, b)#await_token?;
                Ok(())
            }
            pub #async_token fn unlink(
                model_a: &#struct_a,
                model_b: &#struct_b,
                client: &dyn Client,
            ) -> welds::errors::Result<()> {
                let a: &#struct_a = model_a;
                let b: &#struct_b = model_b;
                #wp::query::link::delete::<#model_struct, _, _>(client, a, b)#await_token?;
                Ok(())
            }
        }
    }
}
