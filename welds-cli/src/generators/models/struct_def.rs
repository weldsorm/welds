use crate::config::{Column, DbProvider, Relation, Table};
use crate::errors::Result;
use crate::generators::db_type_lookup::TypeInfo;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use rust_format::{Formatter, RustFmt};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

pub(crate) fn generate(
    mod_path: &PathBuf,
    table: &Table,
    all: &[Table],
    hide_unknown_types: bool,
) -> Result<()> {
    let mut path = PathBuf::from(mod_path);
    path.push("definition.rs");

    let struct_name = format_ident!("{}", table.struct_name());

    let weldstable = build_welds_table(table);
    let relations = build_relations(table, all);
    let fields = build_fields(table, table.database, hide_unknown_types);

    let code = quote! {
        use welds::WeldsModel;

        #[derive(Debug, WeldsModel)]
        #weldstable
        #relations
        pub struct #struct_name {
            #fields
        }
    };

    let mut file = File::create(path)?;
    let formated = RustFmt::default().format_str(code.to_string()).unwrap();
    let formated = format!("{}\n\n{}", super::GENERATED_WARNING, formated);
    file.write_all(formated.as_bytes())?;
    Ok(())
}

fn build_welds_db(providers: &[DbProvider]) -> TokenStream {
    let mut list = Vec::default();
    for p in providers {
        use DbProvider::*;
        list.push(match p {
            Mysql => quote! { db(Mysql) },
            Mssql => quote! { db(Mssql) },
            Postgres => quote! { db(Postgres) },
            Sqlite => quote! { db(Sqlite) },
        });
    }
    quote! { #[welds( #(#list),* )] }
}

fn build_welds_table(table: &Table) -> TokenStream {
    let schema = match &table.schema {
        Some(s) => quote! { schema = #s, },
        None => quote! {},
    };
    let tablename = table.name.as_str();
    quote! { #[welds( #schema table=#tablename )] }
}

fn build_relations(table: &Table, all: &[Table]) -> TokenStream {
    let mut list = Vec::default();
    let hm = quote::format_ident!("HasMany");
    let bt = quote::format_ident!("BelongsTo");
    for relation in &table.has_many {
        if let Some(q) = build_relation(&hm, relation, all) {
            list.push(q);
        }
    }
    for relation in &table.belongs_to {
        if let Some(q) = build_relation(&bt, relation, all) {
            list.push(q);
        }
    }
    quote! { #(#list)* }
}

fn build_relation(ty: &Ident, relation: &Relation, all: &[Table]) -> Option<TokenStream> {
    let other_table = match find_table(&relation.schema, &relation.tablename, all) {
        Some(x) => x,
        None => {
            log::warn!("Relation table Not Found: {:?}", relation);
            return None;
        }
    };
    let struct_name = Ident::new(&other_table.struct_name(), Span::call_site());
    let mod_name = Ident::new(&other_table.module_name(), Span::call_site());
    let fk = &relation.foreign_key;
    Some(quote! { #[welds(#ty(#mod_name, super::super::#mod_name::#struct_name, #fk))] })
}

fn find_table<'a>(
    schema: &'a Option<String>,
    name: &'a str,
    all: &'a [Table],
) -> Option<&'a Table> {
    all.iter().find(|&t| t.name == name && &t.schema == schema)
}

fn build_fields(table: &Table, db: DbProvider, hide_unknown_types: bool) -> TokenStream {
    let mut list = Vec::default();
    for col in &table.columns {
        if let Some(f) = build_field(col, db, hide_unknown_types) {
            list.push(f);
        }
    }
    quote! { #(#list), * }
}

fn build_field(column: &Column, db: DbProvider, hide_unknown_types: bool) -> Option<TokenStream> {
    let mut parts = Vec::default();
    if column.primary_key {
        parts.push(quote! { #[welds(primary_key)]});
    }
    if !column.writeable {
        parts.push(quote! { #[welds(readonly)]});
    }
    let mn = crate::generators::name_sanitize(&column.model_name);
    if mn != column.db_name {
        let dbname = &column.db_name;
        parts.push(quote! { #[sqlx(rename = #dbname)] });
    }
    let span = Span::call_site();
    let f = Ident::new(&mn, span);
    let typeinfo = match crate::generators::db_type_lookup::get(&column.db_type, db) {
        Some(s) => s,
        None => {
            log::warn!("NO DB TYPE FOR: {} {}", f, column.db_type);
            if unknown_types {
                let span = Span::call_site();
                let f = Ident::new(&column.db_type, span);
                TypeInfo {
                    quote: quote!(#f),
                    force_null: false,
                }
            } else {
                return None;
            }
        }
    };

    let force_null = typeinfo.force_null;
    let mut f_type = typeinfo.quote;
    if force_null || column.is_null {
        f_type = quote! {Option<#f_type>};
    }

    parts.push(quote! { pub #f: #f_type });
    Some(quote! { #(#parts)* })
}
