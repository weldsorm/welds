use crate::info::Info;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn write(_infos: &Info) -> TokenStream {
    quote! {

    pub async fn truncate<C, DB>(conn: &C) -> welds::errors::Result<()>
    where
        C: welds::connection::Connection<DB>,
        DB: sqlx::Database,
    {
        let nameparts =
            <<Self as welds::table::HasSchema>::Schema as welds::table::TableInfo>::identifier();
        let identifier = nameparts.join(".");
        let sql = format!("TRUNCATE {}", identifier);
        conn.execute(&sql, Default::default()).await?;
        Ok(())
    }


    }
}
