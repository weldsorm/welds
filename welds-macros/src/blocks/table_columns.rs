use crate::info::Info;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub(crate) fn write(info: &Info) -> TokenStream {
    let columns = write_cols(info);
    let pks = write_pks(info);

    let parts: Vec<_> = info
        .engines_ident
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
    db: &Ident,
    pks: &TokenStream,
    columns: &TokenStream,
) -> TokenStream {
    let def = &info.schemastruct;

    quote! {

        impl welds::table::TableColumns<sqlx::#db> for #def {
            fn primary_keys() -> Vec<welds::table::Column> {
                type DB = sqlx::#db;
                use welds::table::Column;
                #pks
            }
            fn columns() -> Vec<welds::table::Column> {
                type DB = sqlx::#db;
                use welds::table::Column;
                #columns
            }
        }

    }
    .into()
}
