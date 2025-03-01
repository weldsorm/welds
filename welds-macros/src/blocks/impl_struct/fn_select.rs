use crate::info::Info;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn write(info: &Info) -> TokenStream {
    let schema = &info.schemastruct;
    let wp = &info.welds_path;

    quote! {

        pub fn select<V, FN: #wp::query::clause::AsFieldName<V>>(
            lam: impl Fn(<Self as #wp::model_traits::HasSchema>::Schema) -> FN,
        ) -> #wp::query::select_cols::SelectBuilder<Self>
        where
            #schema: #wp::model_traits::TableColumns,
            Self: Send,
        {
            let qb = #wp::query::builder::QueryBuilder::new();
            qb.select(lam)
        }

        pub fn select_as<V, FN: #wp::query::clause::AsFieldName<V>>(
            lam: impl Fn(<Self as #wp::model_traits::HasSchema>::Schema) -> FN,
            as_name: &'static str,
        ) -> #wp::query::select_cols::SelectBuilder<Self>
        where
            #schema: #wp::model_traits::TableColumns,
            Self: Send,
        {
            let qb = #wp::query::builder::QueryBuilder::new();
            qb.select_as(lam, as_name)
        }


    }
}
