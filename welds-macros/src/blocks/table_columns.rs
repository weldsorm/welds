use crate::info::Info;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn write(info: &Info) -> TokenStream {
    let columns = write_cols(info);
    let pks = write_pks(info);
    write_for_db(info, &pks, &columns)
}

pub(crate) fn write_cols(info: &Info) -> TokenStream {
    let parts: Vec<_> = info
        .columns
        .iter()
        .filter(|x| !x.ignore)
        .map(|c| {
            let ft = &c.field_type;
            let ty = quote! { #ft };
            let nullable = c.is_option;
            //if nullable {
            //    ty = quote! { Option<#ty> };
            //}
            let dbname = c.dbname.as_str();
            let rust_type = ty.to_string();
            quote! { Column::new(#dbname, #rust_type, #nullable) }
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
            let ty = quote! { #ft };
            let nullable = c.is_option;
            let dbname = c.dbname.as_str();
            let rust_type = ty.to_string();
            quote! { Column::new(#dbname, #rust_type, #nullable) }
        })
        .collect();
    quote! { vec![ #(#parts),* ] }
}

pub(crate) fn write_for_db(info: &Info, pks: &TokenStream, columns: &TokenStream) -> TokenStream {
    let wp = &info.welds_path;
    let def = &info.schemastruct;

    quote! {

        impl #wp::model_traits::TableColumns for #def {
            fn primary_keys() -> Vec<#wp::model_traits::Column> {
                use #wp::model_traits::Column;
                #pks
            }
            fn columns() -> Vec<#wp::model_traits::Column> {
                use #wp::model_traits::Column;
                #columns
            }
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_define_the_columns_and_pks_on_the_schemamodel() {
        let info = Info::mock()
            .add_pk("id", "i64")
            .add_column("num", "f32", true);
        let ts = write(&info);
        let code = ts.to_string();

        let expected: &str = r#"
            impl welds::model_traits::TableColumns for MockSchema {
                fn primary_keys() -> Vec<welds::model_traits::Column> {
                    use welds::model_traits::Column;
                    vec![Column::new("id", "i64" , false)]
                }
                fn columns() -> Vec<welds::model_traits::Column> {
                    use welds::model_traits::Column;
                    vec![Column::new("id", "i64", false), Column::new("num", "f32",  true)]
                }
            }
        "#;

        eprintln!("CODE: \n{}\n", code);
        assert_eq!(cleaned(&code), cleaned(expected));
    }

    fn cleaned(input: &str) -> String {
        input.chars().filter(|c| !c.is_whitespace()).collect()
    }
}
