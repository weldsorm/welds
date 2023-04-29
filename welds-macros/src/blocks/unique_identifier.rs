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

    let parts: Vec<_> = info
        .engines_ident
        .iter()
        .map(|db| write_for_db(info, db, pk))
        .collect();
    quote! { #(#parts)* }
}

pub(crate) fn write_for_db(info: &Info, db: &Ident, pk: &Column) -> TokenStream {
    let wp = &info.welds_path;
    let def = &info.defstruct;
    let pktype = &pk.field_type;
    let name = &pk.dbname;
    let nullable = pk.is_option;

    quote! {
        impl #wp::table::UniqueIdentifier<sqlx::#db> for #def {
            fn id_column() -> #wp::table::Column {
                #wp::table::Column::new::<sqlx::#db, #pktype>(#name, #nullable)
            }
        }
    }
}
