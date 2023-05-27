use crate::info::Info;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn write(info: &Info) -> TokenStream {
    let schema = &info.schemastruct;
    let wp = &info.welds_path;

    quote! {

        pub fn all<'args, DB>() -> #wp::query::builder::QueryBuilder<'args, Self, DB>
            where
            DB: #wp::connection::Database,
            #schema: #wp::table::TableColumns<DB>,
            Self: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row>,
            {
                #wp::query::builder::QueryBuilder::new()
            }

    }
}
