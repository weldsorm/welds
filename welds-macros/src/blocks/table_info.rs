use crate::info::Info;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn write(info: &Info) -> TokenStream {
    let wp = &info.welds_path;
    let mut parts = Vec::default();
    let tn = &info.tablename;
    parts.push(quote! { #tn });

    if let Some(namespace) = &info.schemaname {
        parts.push(quote! { #namespace });
    }

    let parts: Vec<_> = parts.drain(..).rev().collect();
    let schema = &info.schemastruct;

    quote! {

        impl #wp::model_traits::TableInfo for #schema {
            fn identifier() -> &'static [&'static str] {
                &[#(#parts),*]
            }
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_write_basic_table_info() {
        let info = Info::mock().add_pk("id", "i64");
        let ts = write(&info);
        let code = ts.to_string();

        let expected: &str = r#"
        impl welds::model_traits::TableInfo for MockSchema {
            fn identifier() -> &'static [&'static str] {
                &[ "daschema","datables"]
            }
        }
        "#;
        assert_eq!(cleaned(&code), cleaned(expected), "CODE: \n\n{}\n\n", code);
    }

    fn cleaned(input: &str) -> String {
        input.chars().filter(|c| !c.is_whitespace()).collect()
    }
}
