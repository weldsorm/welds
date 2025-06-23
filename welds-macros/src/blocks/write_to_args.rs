use crate::column::Column;
use crate::info::Info;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn write(info: &Info) -> TokenStream {
    // If this is a readonly model it should NOT impl WriteToArgs
    if info.readonly {
        return quote!();
    }

    let fields: Vec<_> = info
        .columns
        .iter()
        .filter(|x| !x.ignore)
        .filter(|x| !x.readonly)
        .map(write_col_normal)
        .collect();
    let fields = quote! { #(#fields)* };

    write_for_db(info, &fields)
}

pub(crate) fn write_col_normal(col: &Column) -> TokenStream {
    let dbname = col.dbname.as_str();
    let field = &col.field;
    quote! { #dbname => args.push(&self.#field), }
}

pub(crate) fn write_for_db(info: &Info, matches: &TokenStream) -> TokenStream {
    let def = &info.defstruct;
    let wp = &info.welds_path;

    quote! {

    impl #wp::model_traits::WriteToArgs for #def {
        fn bind<'s, 'c, 'a, 'p>(
            &'s self,
            column: &'c str,
            args: &'a mut #wp::query::clause::ParamArgs<'p>,
        ) -> #wp::errors::Result<()>
    where
            's: 'p,
        {

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_write_basic_schemamodel_def() {
        let info = Info::mock().add_pk("id", "i64");
        let ts = write(&info);
        let code = ts.to_string();

        let expected: &str = r#"
            impl welds::model_traits::WriteToArgs for Mock {
                fn bind<'s, 'c, 'a, 'p>(
                    &'s self,
                    column: &'c str,
                    args: &'a mut welds::query::clause::ParamArgs<'p>,
                ) -> welds::errors::Result<()>
                where
                    's: 'p,
                {
                    match column {
                        "id" => args.push(&self.id),
                        _ => {
                            return Err(welds::errors::WeldsError::MissingDbColumn(
                                column.to_owned(),
                            ).into())
                        }
                    }
                    Ok(())
                }
            }

        "#;

        assert_eq!(cleaned(&code), cleaned(expected));
    }

    fn cleaned(input: &str) -> String {
        input.chars().filter(|c| !c.is_whitespace()).collect()
    }
}
