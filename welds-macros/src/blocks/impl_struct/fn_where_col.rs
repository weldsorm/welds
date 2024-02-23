use crate::info::Info;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn write(info: &Info) -> TokenStream {
    let schema = &info.schemastruct;
    let wp = &info.welds_path;

    quote! {



    pub fn where_col(
        lam: impl Fn(#schema) -> Box<dyn #wp::query::clause::ClauseAdder>,
    ) -> #wp::query::builder::QueryBuilder<Self>
    where
        #schema: #wp::model_traits::TableColumns,
        Self: Send
    {
        let select = #wp::query::builder::QueryBuilder::new();
        select.where_col(lam)
    }




    }
}
