use crate::info::Info;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn write(info: &Info) -> TokenStream {
    let wp = &info.welds_path;
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

    pub #async_token fn from_raw_sql<'args, 't>(
        sql: &'static str,
        arguments: &'args #wp::query::clause::ParamArgs<'t>,
        client: &dyn #wp::Client,
    ) -> #wp::errors::Result<Vec<#wp::state::DbState<Self>>>
    where
        <Self as #wp::model_traits::HasSchema>::Schema: #wp::model_traits::TableInfo + #wp::model_traits::TableColumns,
        Self: Send + TryFrom<#wp::Row>,
        #wp::WeldsError: From<<Self as TryFrom<#wp::Row>>::Error>
    {
        let mut rows: Vec<#wp::Row> = client.fetch_rows(sql, arguments)#await_token?;
        let mut data: std::result::Result<Vec<Self>, _> = rows.drain(..).map( Self::try_from ).collect();
        let mut data = data?;

        Ok(data
            .drain(..)
            .map(|x| #wp::state::DbState::db_loaded(x))
            .collect())
    }

    }
}
