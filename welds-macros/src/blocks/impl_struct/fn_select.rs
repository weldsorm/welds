use crate::info::Info;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn write(info: &Info) -> TokenStream {
    let schema = &info.schemastruct;
    let wp = &info.welds_path;

    quote! {

        pub fn select<'args, DB,  V, FN: #wp::query::clause::AsFieldName<V>>(
            lam: impl Fn(<Self as #wp::table::HasSchema>::Schema) -> FN,
        ) -> #wp::query::select_cols::SelectBuilder<'args, Self, DB>
        where
            DB: #wp::connection::Database,
            #schema: #wp::table::TableColumns<DB>,
            Self: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row>,
        {
            let qb = #wp::query::builder::QueryBuilder::new();
            qb.select(lam)
        }


    }
}
