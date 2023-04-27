use crate::info::Info;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn write(infos: &Info) -> TokenStream {
    let schema = &infos.schemastruct;

    quote! {

        pub fn all<'args, DB>() -> welds::query::builder::QueryBuilder<'args, Self, DB>
            where
            DB: sqlx::Database,
            #schema: welds::table::TableColumns<DB>,
            Self: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row>,
            {
                welds::query::builder::QueryBuilder::new()
            }

    }
}
