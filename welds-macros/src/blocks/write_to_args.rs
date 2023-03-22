use crate::column::Column;
use crate::info::Info;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub(crate) fn write(info: &Info) -> TokenStream {
    // If this is a readonly model it should NOT impl WriteToArgs
    if info.readonly {
        return quote!();
    }

    let matches: Vec<_> = info
        .columns
        .iter()
        .filter(|x| !x.ignore)
        .map(|col| write_col(col))
        .collect();

    let matches = quote! { #(#matches)* };
    let parts: Vec<_> = info
        .engines_ident
        .iter()
        .map(|db| write_for_db(info, db, &matches))
        .collect();
    quote! { #(#parts)* }
}

pub(crate) fn write_col(col: &Column) -> TokenStream {
    let dbname = col.dbname.as_str();
    let field = &col.field;
    quote! { #dbname => args.add(&self.#field), }
}

pub(crate) fn write_for_db(info: &Info, db: &Ident, matches: &TokenStream) -> TokenStream {
    let def = &info.defstruct;

    quote! {

    impl welds_core::table::WriteToArgs<sqlx::#db> for #def {
        fn bind<'args>(
            &self,
            column: &str,
            args: &mut <sqlx::#db as sqlx::database::HasArguments<'args>>::Arguments,
        ) -> Result<(), welds_core::errors::WeldsError> {
            use sqlx::Arguments;
            match column {
                #matches
                _ => {
                    return Err(welds_core::errors::WeldsError::MissingDbColumn(
                        column.to_owned(),
                    ))
                }
            }
            Ok(())
        }
    }

        }
    .into()
}
