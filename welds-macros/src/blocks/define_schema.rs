use crate::column::Column;
use crate::info::Info;
use proc_macro2::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;

pub(crate) fn write(info: &Info) -> TokenStream {
    let name = &info.schemastruct;

    let fields: Vec<_> = info
        .columns
        .iter()
        .filter(|x| !x.ignore)
        .map(|x| def_field(info, x))
        .collect();

    let default_fields: Vec<_> = info
        .columns
        .iter()
        .filter(|x| !x.ignore)
        .map(|x| default_fields(info, x))
        .collect();

    quote! {

        pub struct #name {
            #(#fields),*
        }

        impl Default for #name {
            fn default() -> Self {
                Self {
                    #(#default_fields),*
                }
            }
        }

    }
}

fn def_field(info: &Info, col: &Column) -> TokenStream {
    let name = &col.field;
    let wp = &info.welds_path;
    let type_inner = &col.field_type;
    let ty = quote! { #type_inner };
    let clause = get_clause(type_inner, col.is_option);
    let full_type = quote! { #wp::query::clause::#clause<#ty> };
    quote! { pub #name: #full_type }
}

fn get_clause(ty: &syn::Type, nullable: bool) -> TokenStream {
    let clasename = crate::utils::get_clause(ty, nullable);
    let id = Ident::new(clasename.as_str(), Span::call_site());
    quote! { #id }
}

fn default_fields(info: &Info, col: &Column) -> TokenStream {
    let wp = &info.welds_path;
    let name = &col.field;
    let type_inner = &col.field_type;
    let clause = get_clause(type_inner, col.is_option);
    let dbname = col.dbname.as_str();
    let fieldname: String = col.field.to_string();
    quote! { #name: #wp::query::clause::#clause::new(#dbname, #fieldname) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_write_basic_schemamodel_def() {
        let info = Info::mock().add_pk("id", "i64");
        let ts = write(&info);
        let code = ts.to_string();

        let expected: &str = r#"
            pub struct MockSchema {
                pub id: welds::query::clause::Numeric<i64>
            }
            impl Default for MockSchema {
                fn default() -> Self {
                    Self {
                        id: welds::query::clause::Numeric::new("id", "id")
                    }
                }
            }
        "#;

        assert_eq!(cleaned(&code), cleaned(expected));
    }

    fn cleaned(input: &str) -> String {
        input.chars().filter(|c| !c.is_whitespace()).collect()
    }
}
