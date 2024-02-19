use crate::column::Column;
use crate::info::Info;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn write(info: &Info) -> TokenStream {
    if info.pks.len() != 1 {
        return quote!();
    }
    let pk = &info.pks[0];

    write_for_db(info, pk)
}

pub(crate) fn write_for_db(info: &Info, pk: &Column) -> TokenStream {
    let wp = &info.welds_path;
    let def = &info.schemastruct;
    let name = &pk.dbname;
    let nullable = pk.is_option;

    quote! {
        impl #wp::model_traits::UniqueIdentifier for #def {
            fn id_column() -> #wp::model_traits::Column {
                #wp::model_traits::Column::new(#name, #nullable)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_able_to_write_uniqueidentifyer() {
        let info = Info::mock().add_pk("id", "i64");
        let ts = write(&info);
        let code = ts.to_string();
        let expected: &str = r#"
            impl welds::model_traits::UniqueIdentifier for MockSchema {
                fn id_column() -> welds::model_traits::Column {
                    welds::model_traits::Column::new("id", false)
                }
            }
        "#;
        assert_eq!(cleaned(&code), cleaned(expected));
    }

    fn cleaned(input: &str) -> String {
        input.chars().filter(|c| !c.is_whitespace()).collect()
    }
}
