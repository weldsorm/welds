use crate::engine;
use crate::engine::Engine;
use crate::errors::Result;
use crate::utils::as_typepath;
use crate::{column::Column, relation::Relation};
use std::collections::HashSet;
use syn::{Attribute, Field, Type};

pub(crate) fn get_columns(ast: &syn::DeriveInput) -> Vec<Column> {
    let struct_def = match &ast.data {
        syn::Data::Struct(d) => d,
        syn::Data::Enum(_) => panic!("Only Structs are supported by WeldsModel"),
        syn::Data::Union(_) => panic!("Only Structs are supported by WeldsModel"),
    };
    let fields = &struct_def.fields;
    fields
        .iter()
        .filter(|f| f.ident.is_some())
        .map(|f| {
            let ignore = is_welds_ignore(&f.attrs);
            let fieldname = f.ident.as_ref().unwrap().to_string();
            let dbname = read_sqlx_rename(f).unwrap_or(fieldname);
            let field_type = as_option_inner(&f.ty);
            let is_option = field_type.is_some();
            let field_type = field_type.unwrap_or(&f.ty).clone();
            let field = f.ident.as_ref().unwrap().clone();
            Column {
                field,
                ignore,
                dbname,
                field_type,
                is_option,
            }
        })
        .collect()
}

pub(crate) fn get_pks(ast: &syn::DeriveInput) -> Vec<Column> {
    let struct_def = match &ast.data {
        syn::Data::Struct(d) => d,
        syn::Data::Enum(_) => panic!("Only Structs are supported by WeldsModel"),
        syn::Data::Union(_) => panic!("Only Structs are supported by WeldsModel"),
    };
    let fields = &struct_def.fields;
    fields
        .iter()
        .filter(|x| !is_welds_ignore(&x.attrs))
        .filter(|x| is_welds_pk(&x.attrs))
        .filter(|f| f.ident.is_some())
        .map(|f| {
            let fieldname = f.ident.as_ref().unwrap().to_string();
            let dbname = read_sqlx_rename(f).unwrap_or(fieldname);
            let field_type = as_option_inner(&f.ty);
            let is_option = field_type.is_some();
            let field_type = field_type.unwrap_or(&f.ty).clone();
            let field = f.ident.as_ref().unwrap().clone();
            Column {
                field,
                ignore: false,
                dbname,
                field_type,
                is_option,
            }
        })
        .collect()
}

fn read_sqlx_rename(field: &Field) -> Option<String> {
    let metas: Vec<_> = field
        .attrs
        .iter()
        .filter_map(|a| a.parse_meta().ok())
        .filter_map(as_metalist)
        .filter(|m| m.path.is_ident("sqlx"))
        .collect();
    // Read out the inner meta from [welds(this, and_this)]
    let inners: Vec<&syn::Meta> = metas.iter().flat_map(as_metalist_nested_meta).collect();
    // find the first rename="name"
    let db_col_name: Option<String> = inners
        .iter()
        .filter_map(|m| as_meta_namevalue_ref(m))
        .filter(|m| m.path.is_ident("rename"))
        .filter_map(|nv| lit_as_litstr(&nv.lit))
        .map(|x| x.value())
        .next();
    db_col_name
}

/// returns the `inner_type`:  the `T` type inside the `Option<T>`
fn as_option_inner(ftype: &Type) -> Option<&Type> {
    let tp = as_typepath(ftype)?;
    let segment = tp.path.segments.first()?;
    if segment.ident != "Option" {
        return None;
    }
    let args = match &segment.arguments {
        syn::PathArguments::AngleBracketed(args) => args,
        _ => return None,
    };
    let inner_type = args.args.first()?;
    let inner = match inner_type {
        syn::GenericArgument::Type(inner_type) => inner_type,
        _ => return None,
    };
    Some(inner)
}

pub(crate) fn get_engines(ast: &syn::DeriveInput) -> Vec<Engine> {
    let metas = welds_meta(&ast.attrs);

    // Read out the inner meta from [welds(this, and_this)]
    let inners: Vec<&syn::Meta> = metas.iter().flat_map(as_metalist_nested_meta).collect();

    // find all the types in db(this, and_this) inners
    let engines: HashSet<Engine> = inners
        .iter()
        .filter_map(|m| as_metalist_ref(m))
        .filter(|m| m.path.is_ident("db"))
        .flat_map(as_metalist_nested_meta)
        .filter_map(Engine::parse)
        .collect();
    let engines: Vec<_> = engines.into_iter().collect();

    // If there are no Dbs selected, default to supporting all that are enabled
    if engines.is_empty() {
        return engine::ALL.to_vec();
    }

    engines
}

pub(crate) fn get_relations(ast: &syn::DeriveInput) -> Result<Vec<Relation>> {
    let metas = welds_meta(&ast.attrs);

    // Read out the inner meta from [welds(this, and_this)]
    let inners: Vec<&syn::Meta> = metas.iter().flat_map(as_metalist_nested_meta).collect();

    let relations1: Result<Vec<_>> = inners
        .iter()
        .filter_map(|m| as_metalist_ref(m))
        .filter(|m| m.path.is_ident("HasMany"))
        .map(|m| Relation::new(m, "HasMany"))
        .collect();
    let mut relations1 = relations1?;
    let relations2: Result<Vec<_>> = inners
        .iter()
        .filter_map(|m| as_metalist_ref(m))
        .filter(|m| m.path.is_ident("BelongsTo"))
        .map(|m| Relation::new(m, "BelongsTo"))
        .collect();
    let mut relations2 = relations2?;
    let relations: Vec<_> = relations1.drain(..).chain(relations2.drain(..)).collect();

    Ok(relations)
}

pub(crate) fn get_scructname(ast: &syn::DeriveInput) -> syn::Ident {
    ast.ident.clone()
}

pub(crate) fn get_tablename(ast: &syn::DeriveInput) -> String {
    let metas = welds_meta(&ast.attrs);
    // Read out the inner meta from [welds(this, and_this)]
    let inners: Vec<&syn::Meta> = metas.iter().flat_map(as_metalist_nested_meta).collect();
    // find the first table="name"
    let tablename: Option<String> = inners
        .iter()
        .filter_map(|m| as_meta_namevalue_ref(m))
        .filter(|m| m.path.is_ident("table"))
        .filter_map(|nv| lit_as_litstr(&nv.lit))
        .map(|x| x.value())
        .next();
    // If the user didn't give use a table name, use the name of the struct lowercased.
    let structname = ast.ident.to_string().to_lowercase();
    tablename.unwrap_or(structname)
}

pub(crate) fn get_schemaname(ast: &syn::DeriveInput) -> Option<String> {
    let metas = welds_meta(&ast.attrs);
    // Read out the inner meta from [welds(this, and_this)]
    let inners: Vec<&syn::Meta> = metas.iter().flat_map(as_metalist_nested_meta).collect();
    // find the first schema="name"
    inners
        .iter()
        .filter_map(|m| as_meta_namevalue_ref(m))
        .filter(|m| m.path.is_ident("schema"))
        .filter_map(|nv| lit_as_litstr(&nv.lit))
        .map(|x| x.value())
        .next()
}

pub(crate) fn get_readonly(ast: &syn::DeriveInput) -> bool {
    let metas = welds_meta(&ast.attrs);
    // Read out the inner meta from [welds(this, and_this)]
    let inners: Vec<&syn::Meta> = metas.iter().flat_map(as_metalist_nested_meta).collect();
    // find the first readonly
    inners.iter().any(|&m| m.path().is_ident("readonly"))
}

fn as_metalist(meta: syn::Meta) -> Option<syn::MetaList> {
    match meta {
        syn::Meta::List(inner) => Some(inner),
        _ => None,
    }
}

fn as_metalist_ref(meta: &syn::Meta) -> Option<&syn::MetaList> {
    match meta {
        syn::Meta::List(inner) => Some(inner),
        _ => None,
    }
}

fn as_meta_namevalue_ref(meta: &syn::Meta) -> Option<&syn::MetaNameValue> {
    match meta {
        syn::Meta::NameValue(inner) => Some(inner),
        _ => None,
    }
}

fn as_metalist_nested_meta(metalist: &syn::MetaList) -> Vec<&syn::Meta> {
    metalist
        .nested
        .iter()
        .filter_map(|inner| match inner {
            syn::NestedMeta::Meta(m) => Some(m),
            _ => None,
        })
        .collect()
}

fn lit_as_litstr(lit: &syn::Lit) -> Option<&syn::LitStr> {
    match lit {
        syn::Lit::Str(s) => Some(s),
        _ => None,
    }
}

/// pull out all the Welds attrs as metalists
fn welds_meta(attrs: &[Attribute]) -> Vec<syn::MetaList> {
    attrs
        .iter()
        .filter_map(|a| a.parse_meta().ok())
        .filter_map(as_metalist)
        .filter(|m| m.path.is_ident("welds"))
        .collect()
}

fn is_welds_ignore(attrs: &[Attribute]) -> bool {
    let metas = welds_meta(attrs);
    // Check if any attr has ignore
    metas
        .iter()
        .flat_map(as_metalist_nested_meta)
        .any(|m| m.path().is_ident("ignore"))
}

fn is_welds_pk(attrs: &[Attribute]) -> bool {
    let metas = welds_meta(attrs);
    metas
        .iter()
        .flat_map(as_metalist_nested_meta)
        .any(|m| m.path().is_ident("primary_key"))
}
