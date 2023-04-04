use crate::info::Info;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn write(infos: &Info) -> TokenStream {
    let defstruct = &infos.defstruct;
    let schemastruct = &infos.schemastruct;

    quote! {

        impl welds::table::HasSchema for #defstruct {
            type Schema = #schemastruct;
        }

    }
}
