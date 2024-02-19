use crate::info::Info;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn write(info: &Info) -> TokenStream {
    let defstruct = &info.defstruct;
    let wp = &info.welds_path;
    let schemastruct = &info.schemastruct;

    quote! {

        impl #wp::model_traits::HasSchema for #defstruct {
            type Schema = #schemastruct;
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_write_impl_for_hasschema_trait() {
        let info = Info::mock().add_pk("id", "i64");
        let ts = write(&info);
        let code = ts.to_string();

        let expected: &str = r#"
            impl welds::model_traits::HasSchema for Mock {
                type Schema = MockSchema;
            }
        "#;
        assert_eq!(cleaned(&code), cleaned(expected));
    }

    fn cleaned(input: &str) -> String {
        input.chars().filter(|c| !c.is_whitespace()).collect()
    }
}
