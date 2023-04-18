use crate::info::Info;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn write(_infos: &Info) -> TokenStream {
    quote! {

    pub async fn from_raw_sql<'a, 'c, DB, C>(
        sql: &'static str,
        arguments: <DB as sqlx::database::HasArguments<'a>>::Arguments,
        conn: &'c C,
    ) -> welds::errors::Result<Vec<welds::state::DbState<Self>>>
    where
        'c: 'a,
        DB: sqlx::Database,
        C: welds::connection::Connection<DB>,
        Self: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row>,
    {
        let mut data: Vec<Self> = conn.fetch_all(sql, arguments).await?;

        Ok(data
            .drain(..)
            .map(|x| welds::state::DbState::db_loaded(x))
            .collect())
    }

    }
}
