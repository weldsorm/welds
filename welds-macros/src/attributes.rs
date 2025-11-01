use crate::errors::Result;
use crate::utils::as_typepath;
use crate::{
    column::Column,
    hook::{Hook, HookKind},
    relation::Relation,
};
use proc_macro2::{Ident, Span};
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
            let fieldname = f.ident.as_ref().unwrap().to_string();
            let dbname = read_rename(f).unwrap_or(fieldname);
            let field_type = as_option_inner(&f.ty);
            let is_option = field_type.is_some();
            let field_type = field_type.unwrap_or(&f.ty).clone();
            let field = f.ident.as_ref().unwrap().clone();

            let ignores = welds_ignores(&f.attrs);

            let selectable = !ignores.contains(&Ignores::Select);
            let updateable = !ignores.contains(&Ignores::Update);
            let insertable = !ignores.contains(&Ignores::Insert);

            Column {
                field,
                selectable,
                updateable,
                insertable,
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
        .filter(|x| !is_welds_full_ignore(&x.attrs))
        .filter(|x| is_welds_pk(&x.attrs))
        .filter(|f| f.ident.is_some())
        .map(|f| {
            let fieldname = f.ident.as_ref().unwrap().to_string();
            let dbname = read_rename(f).unwrap_or(fieldname);
            let field_type = as_option_inner(&f.ty);
            let is_option = field_type.is_some();
            let field_type = field_type.unwrap_or(&f.ty).clone();
            let field = f.ident.as_ref().unwrap().clone();
            Column {
                field,

                selectable: true,
                // The PK column is used when inserting but not ever updated.
                // This is handled within welds "core"
                // We are marking it as updateable and insertable logic exists for bulk
                insertable: true,
                updateable: true,

                dbname,
                field_type,
                is_option,
            }
        })
        .collect()
}

fn read_rename(field: &Field) -> Option<String> {
    let metas: Vec<_> = field
        .attrs
        .iter()
        .filter_map(|a| a.parse_meta().ok())
        .filter_map(as_metalist)
        .filter(|m| m.path.is_ident("welds"))
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

pub(crate) fn get_relations(ast: &syn::DeriveInput) -> Result<Vec<Relation>> {
    let metas = welds_meta(&ast.attrs);

    // Read out the inner meta from [welds(this, and_this)]
    let inners: Vec<&syn::Meta> = metas.iter().flat_map(as_metalist_nested_meta).collect();

    let relations1: Result<Vec<_>> = inners
        .iter()
        .filter_map(|m| as_metalist_ref(m))
        .filter(|m| m.path.is_ident("HasMany"))
        .map(Relation::basic)
        .collect();
    let mut relations1 = relations1?;

    let relations2: Result<Vec<_>> = inners
        .iter()
        .filter_map(|m| as_metalist_ref(m))
        .filter(|m| m.path.is_ident("BelongsTo"))
        .map(Relation::basic)
        .collect();
    let mut relations2 = relations2?;

    let relations3: Result<Vec<_>> = inners
        .iter()
        .filter_map(|m| as_metalist_ref(m))
        .filter(|m| m.path.is_ident("HasOne"))
        .map(Relation::basic)
        .collect();
    let mut relations3 = relations3?;

    let relations4: Result<Vec<_>> = inners
        .iter()
        .filter_map(|m| as_metalist_ref(m))
        .filter(|m| m.path.is_ident("ManualRelationship"))
        .map(Relation::new_manual)
        .collect();
    let mut relations4 = relations4?;

    let relations5: Result<Vec<_>> = inners
        .iter()
        .filter_map(|m| as_metalist_ref(m))
        .filter(|m| m.path.is_ident("JoinTable"))
        .map(Relation::build_jointable)
        .collect();
    let mut relations5 = relations5?;

    let relations: Vec<_> = relations1
        .drain(..)
        .chain(relations2.drain(..))
        .chain(relations3.drain(..))
        .chain(relations4.drain(..))
        .chain(relations5.drain(..))
        .flatten()
        .collect();

    Ok(relations)
}

pub(crate) fn get_hooks(ast: &syn::DeriveInput) -> Result<Vec<Hook>> {
    let metas = welds_meta(&ast.attrs);

    // Read out the inner meta from [welds(this, and_this)]
    let inners: Vec<&syn::Meta> = metas.iter().flat_map(as_metalist_nested_meta).collect();

    let before_create: Result<Vec<_>> = inners
        .iter()
        .filter_map(|m| as_metalist_ref(m))
        .filter(|m| m.path.is_ident("BeforeCreate"))
        .map(|m| Hook::new(m, HookKind::BeforeCreate))
        .collect();
    let after_create: Result<Vec<_>> = inners
        .iter()
        .filter_map(|m| as_metalist_ref(m))
        .filter(|m| m.path.is_ident("AfterCreate"))
        .map(|m| Hook::new(m, HookKind::AfterCreate))
        .collect();

    let before_update: Result<Vec<_>> = inners
        .iter()
        .filter_map(|m| as_metalist_ref(m))
        .filter(|m| m.path.is_ident("BeforeUpdate"))
        .map(|m| Hook::new(m, HookKind::BeforeUpdate))
        .collect();
    let after_update: Result<Vec<_>> = inners
        .iter()
        .filter_map(|m| as_metalist_ref(m))
        .filter(|m| m.path.is_ident("AfterUpdate"))
        .map(|m| Hook::new(m, HookKind::AfterUpdate))
        .collect();

    let before_delete: Result<Vec<_>> = inners
        .iter()
        .filter_map(|m| as_metalist_ref(m))
        .filter(|m| m.path.is_ident("BeforeDelete"))
        .map(|m| Hook::new(m, HookKind::BeforeDelete))
        .collect();
    let after_delete: Result<Vec<_>> = inners
        .iter()
        .filter_map(|m| as_metalist_ref(m))
        .filter(|m| m.path.is_ident("AfterDelete"))
        .map(|m| Hook::new(m, HookKind::AfterDelete))
        .collect();

    let hooks: Vec<_> = vec![
        before_create?,
        after_create?,
        before_update?,
        after_update?,
        before_delete?,
        after_delete?,
    ]
    .drain(..)
    .flatten()
    .collect();
    Ok(hooks)
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

pub(crate) fn get_welds_path(ast: &syn::DeriveInput) -> syn::Path {
    let metas = welds_path_meta(&ast.attrs);
    // Read out the inner meta from [welds_path(this, and_this)]
    let inners: Vec<&syn::Meta> = metas.iter().flat_map(as_metalist_nested_meta).collect();
    // find the first table="name"
    let first: Option<_> = inners.iter().map(|x| x.path()).next().cloned();
    first.unwrap_or_else(|| {
        let ident = Ident::new("welds", Span::call_site());
        ident.into()
    })
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

/// return the inner nested list "welds(bla, bla2)" -> vec![bla, bla2]
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

/// pull out all the welds_path attrs as metalists
fn welds_path_meta(attrs: &[Attribute]) -> Vec<syn::MetaList> {
    attrs
        .iter()
        .filter_map(|a| a.parse_meta().ok())
        .filter_map(as_metalist)
        .filter(|m| m.path.is_ident("welds_path"))
        .collect()
}

fn welds_ignores(attrs: &[Attribute]) -> Vec<Ignores> {
    if is_welds_full_ignore(attrs) {
        return vec![Ignores::Select, Ignores::Update, Ignores::Insert];
    }
    if is_welds_col_readonly(attrs) {
        return vec![Ignores::Update, Ignores::Insert];
    }
    welds_ignore_subs(attrs)
}

fn is_welds_full_ignore(attrs: &[Attribute]) -> bool {
    let metas = welds_meta(attrs);
    // Check if any attr has ignore
    metas
        .iter()
        //.filter(|m| m.nested.is_empty())
        .flat_map(as_metalist_nested_meta)
        .filter(|m| m.path().is_ident("ignore"))
        // Make sure this "ignore doesn't have children"
        .filter(|m| !matches!(m, syn::Meta::List(_)))
        .any(|_| true)
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Ignores {
    Select,
    Update,
    Insert,
}

impl From<&syn::Path> for Ignores {
    fn from(path: &syn::Path) -> Self {
        let err_msg = "Valid ignore options are (select, update, insert) or empty \n #[welds(ignore)] \n #[welds(ignore(insert))]";
        let idnt = path.get_ident().map(|i| i.to_string()).expect(err_msg);
        match idnt.as_str() {
            "select" => Ignores::Select,
            "update" => Ignores::Update,
            "insert" => Ignores::Insert,
            _ => panic!("{}", err_msg),
        }
    }
}

/// returns the inner contents of a #welds(ignore(bla, bla2)) -> bla, bla2
fn welds_ignore_subs(attrs: &[Attribute]) -> Vec<Ignores> {
    let metas = welds_meta(attrs);
    // Check if any attr has ignore
    metas
        .iter()
        //.filter(|m| m.nested.is_empty())
        .flat_map(as_metalist_nested_meta)
        .filter(|m| m.path().is_ident("ignore"))
        // Make sure this "ignore doesn't have children"
        .flat_map(|m| match m {
            syn::Meta::List(l) => Some(l),
            _ => None,
        })
        .flat_map(|m| m.nested.iter())
        .flat_map(|m| match m {
            syn::NestedMeta::Meta(m) => Some(m),
            _ => None,
        })
        .map(|m| m.path().into())
        .collect()
}

fn is_welds_col_readonly(attrs: &[Attribute]) -> bool {
    let metas = welds_meta(attrs);
    // Check if any attr has are readonly
    metas
        .iter()
        .flat_map(as_metalist_nested_meta)
        .any(|m| m.path().is_ident("readonly"))
}

fn is_welds_pk(attrs: &[Attribute]) -> bool {
    let metas = welds_meta(attrs);
    metas
        .iter()
        .flat_map(as_metalist_nested_meta)
        .any(|m| m.path().is_ident("primary_key"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn should_find_attr_ignore_full() {
        let attr1: Attribute = parse_quote!(#[welds(ignore)]);
        assert!(is_welds_full_ignore(&[attr1]));
    }

    #[test]
    fn should_not_find_attr_ignore() {
        let attr1: Attribute = parse_quote!(#[cars(ignore)]);
        assert!(!is_welds_full_ignore(&[attr1]));
    }

    #[test]
    fn should_find_attr_readonly() {
        let attr1: Attribute = parse_quote!(#[welds(readonly)]);
        assert!(is_welds_col_readonly(&[attr1]));
    }

    #[test]
    fn should_find_attr_ignore_insert() {
        let attr1: Attribute = parse_quote!(#[welds(ignore(insert, select, update))]);
        let subs = welds_ignore_subs(&[attr1]);
        assert!(subs.contains(&Ignores::Insert));
        assert!(subs.contains(&Ignores::Select));
        assert!(subs.contains(&Ignores::Update));
    }
}
