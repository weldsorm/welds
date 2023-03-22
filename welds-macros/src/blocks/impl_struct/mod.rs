use crate::info::Info;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) mod fn_all;
pub(crate) mod fn_new;
pub(crate) mod fn_where;

pub(crate) fn write(infos: &Info) -> TokenStream {
    let defstruct = &infos.defstruct;

    let p1 = fn_new::write(infos);
    let p2 = fn_all::write(infos);
    let p3 = fn_where::write(infos);

    quote! {

        impl #defstruct {
            #p1
            #p2
            #p3
        }

    }
}
