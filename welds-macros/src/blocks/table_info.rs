use crate::info::Info;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn write(info: &Info) -> TokenStream {
    let identifier = match &info.schemaname {
        Some(s) => format!("{}.{}", s, info.tablename),
        None => format!("{}", info.tablename),
    };
    let schema = &info.schemastruct;

    quote! {

        impl welds::table::TableInfo for #schema {
            fn identifier() -> &'static str {
                #identifier
            }
        }

    }
}
