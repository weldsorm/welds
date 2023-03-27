use crate::column::Column;
use crate::info::Info;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub(crate) fn write(info: &Info) -> TokenStream {
    if info.pks.len() != 1 {
        return quote!();
    }
    let pk = &info.pks[0];

    quote! {}
}

//pub(crate) fn write_cols(info: &Info) -> TokenStream {
//    let parts: Vec<_> = info
//        .columns
//        .iter()
//        .filter(|x| !x.ignore)
//        .map(|c| {
//            let ft = &c.field_type;
//            let mut ty = quote! { #ft };
//            if c.is_option {
//                ty = quote! { Option<#ty> };
//            }
//            let dbname = c.dbname.as_str();
//            quote! { Column::new::<DB, #ty>(#dbname) }
//        })
//        .collect();
//    quote! { vec![ #(#parts),* ] }
//}
//
//pub(crate) fn write_pks(info: &Info) -> TokenStream {
//    let parts: Vec<_> = info
//        .pks
//        .iter()
//        .filter(|x| !x.ignore)
//        .map(|c| {
//            let ft = &c.field_type;
//            let mut ty = quote! { #ft };
//            if c.is_option {
//                ty = quote! { Option<#ty> };
//            }
//            let dbname = c.dbname.as_str();
//            quote! { Column::new::<DB, #ty>(#dbname) }
//        })
//        .collect();
//    quote! { vec![ #(#parts),* ] }
//}

pub(crate) fn write_for_db(
    info: &Info,
    db: &Ident,
    pks: &TokenStream,
    columns: &TokenStream,
) -> TokenStream {
    let def = &info.schemastruct;

    quote! {

        impl welds_core::table::TableColumns<sqlx::#db> for #def {
            fn primary_keys() -> Vec<welds_core::table::Column> {
                type DB = sqlx::#db;
                use welds_core::table::Column;
                #pks
            }
            fn columns() -> Vec<welds_core::table::Column> {
                type DB = sqlx::#db;
                use welds_core::table::Column;
                #columns
            }
        }

    }
    .into()
}
