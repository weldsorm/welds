use proc_macro2::TokenTree;
use syn::Ident;

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
    pub(crate) foreign_struct: syn::Ident,
    // the field name (on the model) of the FK in the DB
    pub(crate) foreign_key_db: String,
    // Used for ManualRelationship, the non-id field defined by developer
    pub(crate) self_key_db: Option<String>,

    // A flag to know this Relation is part of a many-to-many jointable
    pub(crate) is_jointable: bool,
}

fn read_as_ident(list: &Vec<TokenTree>, index: usize) -> Option<syn::Ident> {
    let field = list.get(index)?;
   match field {
        TokenTree::Ident(ident) => {
            let ident_str = ident.to_string();
            Some(syn::Ident::new(&ident_str, ident.span()))
        }
        _ => None,
    }
}

fn read_as_string(list: &Vec<TokenTree>, index: usize) -> Option<String> {
    // If we have metas, try to extract string from NameValue
    let meta = list.get(index)?;
    match meta {
        TokenTree::Literal(lit) => Some(lit.to_string().trim_matches('"').to_string()),
        _ => None,
    }
}
