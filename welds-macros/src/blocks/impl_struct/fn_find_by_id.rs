use crate::column::Column;
use crate::info::Info;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn write(info: &Info) -> TokenStream {
    let pks = info.pks.as_slice();
    let wp = &info.welds_path;
    if pks.is_empty() {
        return quote!();
    }

    let id_params: Vec<_> = pks.iter().map(id_param).collect();
    let id_params = quote! { #(#id_params),* };

    let converts: Vec<_> = pks.iter().map(convert).collect();
    let converts = quote! {#(#converts)* };

    let filters: Vec<_> = pks.iter().map(filter).collect();
    let filters = quote! {#(#filters)* };

    quote! {

    pub async fn find_by_id(
        conn: &dyn #wp::Client,
        #id_params
    ) -> #wp::errors::Result<Option<#wp::state::DbState<Self>>>
    where
        <Self as #wp::model_traits::HasSchema>::Schema: #wp::model_traits::TableColumns,
        Self: Send + TryFrom<#wp::Row>,
    {
        #converts
        let mut q = Self::all();
        #filters
        let mut results = q.limit(1).run(conn).await?;
        Ok(results.pop())
    }

    }
}

fn id_param(col: &Column) -> TokenStream {
    let name = &col.field;
    let ty = &col.field_type;
    quote! { #name: impl Into<#ty> }
}

fn filter(col: &Column) -> TokenStream {
    let name = &col.field;
    quote! { q = q.where_col(|x| x.#name.equal(#name.clone())); }
}

fn convert(col: &Column) -> TokenStream {
    let name = &col.field;
    let ty = &col.field_type;
    quote! { let #name: #ty = #name.into(); }
}
