use crate::info::Info;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn write(info: &Info) -> TokenStream {
    let wp = &info.welds_path;
    quote! {

    pub async fn from_raw_sql<'args, 't, 'c, C>(
        sql: &'static str,
        arguments: &'args #wp::query::clause::ParamArgs<'t>,
        client: &'c C,
    ) -> #wp::errors::Result<Vec<#wp::state::DbState<Self>>>
    where
        C: #wp::Client,
        <Self as #wp::model_traits::HasSchema>::Schema: #wp::model_traits::TableInfo + #wp::model_traits::TableColumns,
        Self: Send + TryFrom<#wp::Row>,
        #wp::WeldsError: From<<Self as TryFrom<#wp::Row>>::Error>
    {
        let mut rows: Vec<#wp::Row> = client.fetch_rows(sql, arguments).await?;
        let mut data: std::result::Result<Vec<Self>, _> = rows.drain(..).map( Self::try_from ).collect();
        let mut data = data?;

        Ok(data
            .drain(..)
            .map(|x| #wp::state::DbState::db_loaded(x))
            .collect())
    }

    }
}
