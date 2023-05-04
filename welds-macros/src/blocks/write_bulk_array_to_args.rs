use crate::column::Column;
use crate::info::Info;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn write(info: &Info) -> TokenStream {
    // If this is a readonly model it should NOT impl WriteToArgs
    if info.readonly || info.columns.is_empty() {
        return quote!();
    }

    let fields: Vec<_> = info
        .columns
        .iter()
        .filter(|x| !x.ignore)
        .map(write_col_if)
        .collect();
    let fields_ifs = quote! { #(#fields)* };

    let types: Vec<_> = info
        .columns
        .iter()
        .filter(|x| !x.ignore)
        .map(|x| &x.field_type)
        .collect();
    let types = uniq(types);
    let where_types: Vec<_> = types.iter().cloned().map(write_where).collect();
    let where_types = quote! { #(#where_types)* };

    write_impl(info, where_types, fields_ifs)
}

fn write_where(ty: &syn::Type) -> TokenStream {
    quote! {
        Vec<#ty>: sqlx::Type<DB> + for<'r> sqlx::Encode<'r, DB>,
        #ty: Clone,
        Vec<Option<#ty>>: sqlx::Type<DB> + for<'r> sqlx::Encode<'r, DB>,
        Option<#ty>: Clone,
    }
}

fn write_col_if(col: &Column) -> TokenStream {
    let dbname = col.dbname.as_str();
    let field = &col.field;
    quote! {
        if #dbname == column.name() {
            let chunk: Vec<_> = data.iter().map(|x| x.#field.clone()).collect();
            args.add(chunk);
            return Ok(());
        }
    }
}

pub(crate) fn write_impl(
    info: &Info,
    where_types: TokenStream,
    fields_ifs: TokenStream,
) -> TokenStream {
    let def = &info.defstruct;
    let wp = &info.welds_path;

    quote! {

    impl<DB: sqlx::Database> #wp::table::WriteBulkArrayToArgs<DB> for #def
    where
        #where_types
    {
        fn bind(
            data: &[&Self],
            column: &#wp::table::Column,
            args: &mut <DB as sqlx::database::HasArguments<'_>>::Arguments,
        ) -> #wp::errors::Result<()> {
            use sqlx::Arguments;
            #fields_ifs
            Err(#wp::errors::WeldsError::MissingDbColumn(
                column.name().to_string(),
            ))?
        }
    }

    }
}

fn uniq<T>(list: Vec<T>) -> Vec<T>
where
    T: std::hash::Hash + std::cmp::Eq,
{
    let mut set = std::collections::HashSet::new();
    for x in list {
        set.insert(x);
    }
    set.drain().collect()
}
