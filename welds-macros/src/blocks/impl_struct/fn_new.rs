use crate::info::Info;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn write(info: &Info) -> TokenStream {
    let wp = &info.welds_path;
    let cols: Vec<_> = info
        .columns
        .iter()
        .map(|c| {
            let name = &c.field;
            quote! { #name: Default::default() }
        })
        .collect();
    let cols = quote! { #(#cols),* };

    quote! {

        pub fn new() -> #wp::state::DbState<Self> {
            #wp::state::DbState::new_uncreated(Self {
                #cols
            })
        }

    }
}
