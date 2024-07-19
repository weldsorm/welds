use crate::column::Column;
use crate::info::Info;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn write(info: &Info) -> TokenStream {
    // If this is a readonly model it should NOT impl ColumnDefaultCheck
    if info.readonly {
        return quote!();
    }

    let fields: Vec<_> = info
        .columns
        .iter()
        .filter(|x| !x.ignore)
        .map(write_col_normal)
        .collect();
    let fields = quote! { #(#fields)* };

    write_for_db(info, &fields)
}

pub(crate) fn write_col_normal(col: &Column) -> TokenStream {
    let dbname = col.dbname.as_str();
    let field = &col.field;
    let field_type = &col.field_type;
    if col.is_option {
        quote! { #dbname => self.#field.is_none(), }
    } else {
        quote! { #dbname => self.#field == #field_type::default(), }
    }
}

pub(crate) fn write_for_db(info: &Info, matches: &TokenStream) -> TokenStream {
    let def = &info.defstruct;
    let wp = &info.welds_path;

    quote! {

    impl #wp::model_traits::ColumnDefaultCheck for #def {
        fn col_is_default<'s, 'c>(
            &'s self,
            column: &'c str,
        ) -> #wp::errors::Result<bool>
        {
            let v = match column {
                #matches
                _ => {
                    return Err(#wp::errors::WeldsError::MissingDbColumn(
                        column.to_owned(),
                    ).into())
                }
            };
            Ok(v)
        }
    }

    }
}
