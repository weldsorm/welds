use crate::info::Info;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn write(infos: &Info) -> TokenStream {
    let schema = &infos.schemastruct;

    quote! {

        pub fn all<'args, DB>() -> welds_core::query::select::SelectBuilder<'args, Self, DB>
            where
            DB: sqlx::Database,
            #schema: welds_core::table::TableColumns<DB>,
            Self: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row>,
            {
                welds_core::query::select::SelectBuilder::new()
            }

    }
}
