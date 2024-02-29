use crate::column::Column;
use crate::info::Info;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn write(info: &Info) -> TokenStream {
    let fields: Vec<_> = info
        .columns
        .iter()
        .filter(|x| !x.ignore)
        .map(setfield)
        .collect();
    let fields = quote! { #(#fields)* };

    // Get all the columns that are ignored
    let ignored: Vec<_> = info.columns.iter().filter(|&x| x.ignore).collect();

    write_for_db(info, &fields, &ignored)
}

pub(crate) fn setfield(col: &Column) -> TokenStream {
    let dbname = col.dbname.as_str();
    let field = &col.field;
    quote! { #field: row.get(#dbname)?, }
}

pub(crate) fn write_for_db(
    info: &Info,
    fieldsets: &TokenStream,
    ignored: &[&Column],
) -> TokenStream {
    let def = &info.defstruct;
    let wp = &info.welds_path;

    // If there are fields that are not connected to the database.
    // Ignore them
    //
    let defaults = if ignored.is_empty() {
        quote! {}
    } else {
        quote! { ..Default::default() }
    };

    quote! {

    impl TryFrom<#wp::Row> for #def {
        type Error = #wp::WeldsError;
        fn try_from(row: #wp::Row) -> std::result::Result<Self, Self::Error> {
            Ok(#def {
                  #fieldsets
                  #defaults
            })
        }
    }

      }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_write_basic_schemamodel_def() {
        let info = Info::mock()
            .add_pk("id", "i64")
            .add_column("name", "String", true);
        let ts = write(&info);
        let code = ts.to_string();

        let expected: &str = r#"
            impl TryFrom<welds::Row> for Mock {
                type Error = welds::WeldsError;
                fn try_from(row: welds::Row) -> std::result::Result<Self, Self::Error> {
                    Ok(Mock {
                        id: row.get("id")?,
                        name: row.get("name")?,
                    })
                }
            }
        "#;

        assert_eq!(cleaned(&code), cleaned(expected));
    }

    fn cleaned(input: &str) -> String {
        input.chars().filter(|c| !c.is_whitespace()).collect()
    }
}
