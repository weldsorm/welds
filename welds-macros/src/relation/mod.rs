use syn::Ident;
use syn::MetaList;

mod basic;
mod jointable;
mod manual;

#[derive(Debug)]
pub(crate) struct Relation {
    // How this relationship was defined (HasMany/BelongsTo/HasOne/Manual)
    pub(crate) kind: Ident,
    // The name of the field in the relationship.
    // How this relationship is referred to by the user
    pub(crate) field: Ident,
    /// The model this relationship links to
    pub(crate) foreign_struct: syn::Path,
    // the field name (on the model) of the FK in the DB
    pub(crate) foreign_key_db: String,
    // Used for ManualRelationship, the non-id field defined by developer
    pub(crate) self_key_db: Option<String>,

    // A flag to know this Relation is part of a many-to-many jointable
    pub(crate) is_jointable: bool,
}

fn read_as_path(list: &MetaList, index: usize) -> Option<syn::Path> {
    let part = list.nested.iter().nth(index)?;
    let meta = match part {
        syn::NestedMeta::Meta(m) => m,
        _ => return None,
    };
    match meta {
        syn::Meta::Path(path) => Some(path.clone()),
        _ => None,
    }
}

fn read_as_ident(list: &MetaList, index: usize) -> Option<syn::Ident> {
    let part = list.nested.iter().nth(index)?;
    let field = match part {
        syn::NestedMeta::Meta(m) => m,
        _ => return None,
    };
    let field = match field {
        syn::Meta::Path(path) => path,
        _ => return None,
    };
    if field.segments.len() != 1 {
        return None;
    }
    let field = field.segments[0].ident.clone();
    Some(field)
}

fn read_as_string(list: &MetaList, index: usize) -> Option<String> {
    let part = list.nested.iter().nth(index)?;
    let lit = match part {
        syn::NestedMeta::Lit(lit) => lit,
        _ => return None,
    };
    let lit_str = match lit {
        syn::Lit::Str(s) => s,
        _ => return None,
    };
    Some(lit_str.value())
}
