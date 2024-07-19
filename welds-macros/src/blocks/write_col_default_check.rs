use crate::column::Column;
use crate::info::Info;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{PathArguments, Type, TypePath};

pub(crate) fn write(info: &Info) -> TokenStream {
    // If this is a readonly model it should NOT impl ColumnDefaultCheck
    if info.readonly {
        return quote!();
    }

    let fields: Vec<_> = info
        .columns
        .iter()
        .filter(|x| !x.ignore)
        .map(col_switch)
        .collect();
    let fields = quote! { #(#fields)* };

    write_default_check_impl(info, &fields)
}

pub(crate) fn col_switch(col: &Column) -> TokenStream {
    let dbname = col.dbname.as_str();
    let field = &col.field;
    let field_type = &col.field_type;

    if col.is_option {
        return quote! { #dbname => self.#field.is_none(), };
    }
    if is_generic_type(field_type) {
        return quote! { #dbname => true, };
    }

    quote! { #dbname => self.#field == #field_type::default(), }
}

pub(crate) fn write_default_check_impl(info: &Info, matches: &TokenStream) -> TokenStream {
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

/// returns true if a given syn::Type is a generic
fn is_generic_type(ty: &Type) -> bool {
    if let Type::Path(TypePath { path, .. }) = ty {
        for segment in &path.segments {
            if let PathArguments::AngleBracketed(_) = &segment.arguments {
                return true;
            }
        }
    }
    false
}
