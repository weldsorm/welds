use crate::info::Info;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn write(infos: &Info) -> TokenStream {
    let cols: Vec<_> = infos
        .columns
        .iter()
        .map(|c| {
            let name = &c.field;
            quote! { #name: Default::default() }
        })
        .collect();
    let cols = quote! { #(#cols),* };

    quote! {

        pub fn new() -> welds::state::DbState<Self> {
            welds::state::DbState::new_uncreated(Self {
                #cols
            })
        }

    }
}
