use crate::info::Info;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn write(infos: &Info) -> TokenStream {
    let schema = &infos.schemastruct;

    quote! {



    pub fn where_col<'args, DB>(
        lam: impl Fn(#schema) -> Box<dyn welds::query::clause::ClauseAdder<'args, DB>>,
    ) -> welds::query::builder::QueryBuilder<'args, Self, DB>
    where
        DB: sqlx::Database,
        #schema: welds::table::TableColumns<DB>,
        Self: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row>,
    {
        let select = welds::query::builder::QueryBuilder::new();
        select.where_col(lam)
    }




    }
}
