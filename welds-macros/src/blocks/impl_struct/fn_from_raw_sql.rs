use crate::info::Info;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn write(infos: &Info) -> TokenStream {
    let schema = &infos.schemastruct;

    quote! {

    pub async fn from_raw_sql<'q, 'schema, 'args, 'e, DB, E, A>(
        sql: &'static str,
        arguments: A,
        exec: E,
    ) -> welds_core::errors::Result<Vec<welds_core::state::DbState<Self>>>
    where
        A: sqlx::IntoArguments<'q, DB> + 'q,
        DB: sqlx::Database,
        E: sqlx::Executor<'e, Database = DB>,
        <DB as sqlx::database::HasArguments<'schema>>::Arguments: sqlx::IntoArguments<'args, DB>,
        Self: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row>,
    {
        let mut data: Vec<Self> = sqlx::query_as_with(sql, arguments).fetch_all(exec).await?;
        Ok(data
            .drain(..)
            .map(|x| welds_core::state::DbState::db_loaded(x))
            .collect())
    }

    }
}
