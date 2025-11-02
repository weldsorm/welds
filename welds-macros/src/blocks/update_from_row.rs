use crate::column::Column;
use crate::info::Info;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn write(info: &Info) -> TokenStream {
    let fields: Vec<_> = info
        .columns
        .iter()
        .filter(|x| x.selectable)
        .map(setfield)
        .collect();
    let fields = quote! { #(#fields)* };

    write_for_db(info, &fields)
}

pub(crate) fn setfield(col: &Column) -> TokenStream {
    let dbname = col.dbname.as_str();
    let field = &col.field;
    quote! { self.#field = row.get(#dbname)?; }
}

pub(crate) fn write_for_db(info: &Info, fieldsets: &TokenStream) -> TokenStream {
    let def = &info.defstruct;
    let wp = &info.welds_path;

    // Updates an existing model from a welds-connection Row

    quote! {
          impl #wp::model_traits::UpdateFromRow for #def {
            fn update_from_row(&mut self, row: &mut #wp::Row) -> #wp::errors::Result<()> {
                #fieldsets
                Ok(())
            }
          }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_able_to_update_a_model_from_a_row() {
        let info = Info::mock()
            .add_pk("id", "i64")
            .add_column("name", "String", true);
        let ts = write(&info);
        let code = ts.to_string();

        let expected: &str = r#"
            impl welds::model_traits::UpdateFromRow for Mock {
              fn update_from_row(&mut self, row: &mut welds::Row) -> welds::errors::Result<()> {
                  self.id= row.get("id")?;
                  self.name= row.get("name")?;
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
