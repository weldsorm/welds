use crate::column::Column;
use crate::info::Info;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn write(info: &Info) -> TokenStream {
    let colstruct = write_colstruct(info);

    let read_columns = read_cols(&info.columns);
    let write_columns = write_cols(&info.columns);

    let pks = write_cols(&info.pks);
    write_for_db(info, &colstruct, &pks, &read_columns, &write_columns)
}

pub(crate) fn write_colstruct(info: &Info) -> TokenStream {
    let wp = &info.welds_path;
    let name = &info.colstruct;

    let fields: Vec<_> = info
        .columns
        .iter()
        .filter(|c| !c.ignore)
        .map(|c| {
            let name = &c.field;
            quote! { pub #name: #wp::model_traits::Column }
        })
        .collect();

    let default_fields: Vec<_> = info
        .columns
        .iter()
        .filter(|x| !x.ignore)
        .map(|c| {
            let name = &c.field;
            let ft = &c.field_type;
            let ty = quote! { #ft };
            let nullable = c.is_option;
            //if nullable {
            //    ty = quote! { Option<#ty> };
            //}
            let dbname = c.dbname.as_str();
            let rust_type = ty.to_string();
            quote! { #name: #wp::model_traits::Column::new(#dbname, #rust_type, #nullable) }
        })
        .collect();

    quote! {
        pub struct #name {
            #(#fields,)*
        }

        impl Default for #name {
            fn default() -> Self {
                Self {
                    #(#default_fields,)*
                }
            }
        }
    }
}

pub(crate) fn read_cols(columns: &[Column]) -> TokenStream {
    let parts: Vec<_> = columns
        .iter()
        .filter(|x| !x.ignore)
        .map(|c| {
            let name = &c.field;
            quote! { columns.#name }
        })
        .collect();
    quote! { vec![ #(#parts),* ] }
}

pub(crate) fn write_cols(columns: &[Column]) -> TokenStream {
    let parts: Vec<_> = columns
        .iter()
        .filter(|x| !x.ignore)
        .filter(|x| !x.readonly)
        .map(|c| {
            let name = &c.field;
            quote! { columns.#name }
        })
        .collect();
    quote! { vec![ #(#parts),* ] }
}

pub(crate) fn write_for_db(
    info: &Info,
    colstruct: &TokenStream,
    pks: &TokenStream,
    read_columns: &TokenStream,
    write_columns: &TokenStream,
) -> TokenStream {
    let wp = &info.welds_path;
    let ident_schemastruct = &info.schemastruct;
    let ident_colstruct = &info.colstruct;

    quote! {
        #colstruct

        impl #wp::model_traits::TableColumns for #ident_colstruct {
            type ColumnStruct = #ident_colstruct;

            fn primary_keys() -> Vec<#wp::model_traits::Column> {
                #[allow(dead_code)]
                let columns = Self::default();
                #pks
            }

            fn readable_columns() -> Vec<#wp::model_traits::Column> {
                #[allow(dead_code)]
                let columns = Self::default();
                #read_columns
            }

            fn writable_columns() -> Vec<#wp::model_traits::Column> {
                #[allow(dead_code)]
                let columns = Self::default();
                #write_columns
            }

        }

        impl #wp::model_traits::TableColumns for #ident_schemastruct {
            type ColumnStruct = #ident_colstruct;

            fn primary_keys() -> Vec<#wp::model_traits::Column> {
                #ident_colstruct::primary_keys()
            }

            fn readable_columns() -> Vec<#wp::model_traits::Column> {
                #ident_colstruct::readable_columns()
            }

            fn writable_columns() -> Vec<#wp::model_traits::Column> {
                #ident_colstruct::writable_columns()
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

        let expected = quote! {
            pub struct MockColumns {
                pub id: welds::model_traits::Column,
                pub num: welds::model_traits::Column,
            }

            impl Default for MockColumns {
                fn default() -> Self {
                    Self {
                        id: welds::model_traits::Column::new("id", "i64", false),
                        num: welds::model_traits::Column::new("num", "f32", true),
                    }
                }
            }

            impl welds::model_traits::TableColumns for MockColumns {
                type ColumnStruct = MockColumns;

                fn primary_keys() -> Vec<welds::model_traits::Column> {
                    #[allow(dead_code)]
                    let columns = Self::default();
                    vec![columns.id]
                }

                fn columns() -> Vec<welds::model_traits::Column> {
                    #[allow(dead_code)]
                    let columns = Self::default();
                    vec![columns.id, columns.num]
                }
            }

            impl welds::model_traits::TableColumns for MockSchema {
                type ColumnStruct = MockColumns;

                fn primary_keys() -> Vec<welds::model_traits::Column> {
                    MockColumns::primary_keys()
                }

                fn columns() -> Vec<welds::model_traits::Column> {
                    MockColumns::columns()
                }
            }
        };

        eprintln!("CODE: \n{ts}\n");
        assert_eq!(ts.to_string(), expected.to_string());
    }
}
