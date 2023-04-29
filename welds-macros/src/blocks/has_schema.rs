use crate::info::Info;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn write(info: &Info) -> TokenStream {
    let defstruct = &info.defstruct;
    let wp = &info.welds_path;
    let schemastruct = &info.schemastruct;

    quote! {

        impl #wp::table::HasSchema for #defstruct {
            type Schema = #schemastruct;
        }

    }
}
