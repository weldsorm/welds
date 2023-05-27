use crate::column::Column;
use crate::info::Info;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashSet;

pub(crate) fn write(info: &Info) -> TokenStream {
    let pks = info.pks.as_slice();
    let wp = &info.welds_path;
    if pks.is_empty() {
        return quote!();
    }

    let id_params: Vec<_> = pks.iter().map(id_param).collect();
    let id_params = quote! { #(#id_params),* };

    let typelist = uniq_type_list(pks);
    let encode_types: Vec<_> = typelist.iter().map(encode_type).collect();
    let encode_types = quote! {#(#encode_types),* };

    let converts: Vec<_> = pks.iter().map(convert).collect();
    let converts = quote! {#(#converts)* };

    let filters: Vec<_> = pks.iter().map(filter).collect();
    let filters = quote! {#(#filters)* };

    quote! {

    pub async fn find_by_id<'a, 'args, DB, C>(
        conn: &'a C,
        #id_params
    ) -> #wp::errors::Result<Option<#wp::state::DbState<Self>>>
    where
        'a: 'args,
        DB: #wp::connection::Database,
        C: #wp::connection::Connection<DB>,
        <Self as #wp::table::HasSchema>::Schema: #wp::table::TableColumns<DB>,
        <DB as sqlx::database::HasArguments<'a>>::Arguments: sqlx::IntoArguments<'args, DB>,
        Self: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row>,
        DB: #wp::writers::DbLimitSkipWriter,
        DB: #wp::writers::DbColumnWriter,
        DB: #wp::query::clause::DbParam,
        #encode_types
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

fn encode_type(id_type: &syn::Type) -> TokenStream {
    quote! { #id_type: sqlx::Encode<'a, DB> + sqlx::Type<DB> }
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

fn uniq_type_list(cols: &[Column]) -> Vec<syn::Type> {
    let mut set: HashSet<syn::Type> = Default::default();
    for col in cols {
        set.insert(col.field_type.clone());
    }
    set.into_iter().collect()
}
