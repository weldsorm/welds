use crate::info::Info;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn write(info: &Info) -> TokenStream {
    let columns = write_cols(info);
    let pks = write_pks(info);

    let parts: Vec<_> = info
        .engines_path
        .iter()
        .map(|db| write_for_db(info, db, &pks, &columns))
        .collect();
    quote! { #(#parts)* }
}

pub(crate) fn write_cols(info: &Info) -> TokenStream {
    let parts: Vec<_> = info
        .columns
        .iter()
        .filter(|x| !x.ignore)
        .map(|c| {
            let ft = &c.field_type;
            let mut ty = quote! { #ft };
            let nullable = c.is_option;
            if nullable {
                ty = quote! { Option<#ty> };
            }
            let dbname = c.dbname.as_str();
            quote! { Column::new::<DB, #ty>(#dbname, #nullable) }
        })
        .collect();
    quote! { vec![ #(#parts),* ] }
}

pub(crate) fn write_pks(info: &Info) -> TokenStream {
    let parts: Vec<_> = info
        .pks
        .iter()
        .filter(|x| !x.ignore)
        .map(|c| {
            let ft = &c.field_type;
            let mut ty = quote! { #ft };
            let nullable = c.is_option;
            if nullable {
                ty = quote! { Option<#ty> };
            }
            let dbname = c.dbname.as_str();
            quote! { Column::new::<DB, #ty>(#dbname, #nullable) }
        })
        .collect();
    quote! { vec![ #(#parts),* ] }
}

pub(crate) fn write_for_db(
    info: &Info,
    db: &syn::Path,
    pks: &TokenStream,
    columns: &TokenStream,
) -> TokenStream {
    let wp = &info.welds_path;
    let def = &info.schemastruct;

    quote! {

        impl #wp::table::TableColumns<#db> for #def {
            fn primary_keys() -> Vec<#wp::table::Column> {
                type DB = #db;
                use #wp::table::Column;
                #pks
            }
            fn columns() -> Vec<#wp::table::Column> {
                type DB = #db;
                use #wp::table::Column;
                #columns
            }
        }

    }
}
