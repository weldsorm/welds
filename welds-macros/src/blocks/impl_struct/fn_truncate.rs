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
    pub #async_token fn truncate<C, DB>(conn: &C) -> #wp::errors::Result<()>
    where
        C: #wp::connection::Connection<DB>,
        DB: #wp::connection::Database,
    {
        let nameparts =
            <<Self as #wp::table::HasSchema>::Schema as #wp::table::TableInfo>::identifier();
        let identifier = nameparts.join(".");
        let sql = format!("TRUNCATE {}", identifier);
        conn.execute(&sql, Default::default())#await_token?;
        Ok(())
    }


    }
}
