use crate::column::Column;
use crate::info::Info;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn write(info: &Info) -> TokenStream {
    // If this is a readonly model it should NOT impl WriteToArgs
    if info.readonly {
        return quote!();
    }

    let mut blocks = Vec::default();

    for db in &info.engines_path {
        let db_path_str = quote!(#db).to_string();
        let write_col = if db_path_str.ends_with("Sqlite") {
            write_col_sqlite
        } else {
            write_col_normal
        };

        let fields: Vec<_> = info
            .columns
            .iter()
            .filter(|x| !x.ignore)
            .map(write_col)
            .collect();
        let fields = quote! { #(#fields)* };

        blocks.push(write_for_db(info, db, &fields));
    }

    quote! { #(#blocks)* }
}

pub(crate) fn write_col_normal(col: &Column) -> TokenStream {
    let dbname = col.dbname.as_str();
    let field = &col.field;
    quote! { #dbname => args.add(&self.#field), }
}

pub(crate) fn write_col_sqlite(col: &Column) -> TokenStream {
    let dbname = col.dbname.as_str();
    let field = &col.field;
    quote! { #dbname => args.add(self.#field.clone()), }
}

pub(crate) fn write_for_db(info: &Info, db: &syn::Path, matches: &TokenStream) -> TokenStream {
    let def = &info.defstruct;
    let wp = &info.welds_path;

    quote! {

    impl #wp::table::WriteToArgs<#db> for #def {
        fn bind<'args>(
            &self,
            column: &str,
            args: &mut <#db as sqlx::database::HasArguments<'args>>::Arguments,
        ) -> #wp::errors::Result<()> {
            use sqlx::Arguments;
            match column {
                #matches
                _ => {
                    return Err(#wp::errors::WeldsError::MissingDbColumn(
                        column.to_owned(),
                    ).into())
                }
            }
            Ok(())
        }
    }

        }
}
