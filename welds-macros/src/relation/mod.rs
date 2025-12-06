use syn::{Expr, Ident};

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

fn read_as_path(list: &Vec<&Expr>, index: usize) -> Option<syn::Path> {
    let part = list.get(index)?;

    match part {
        Expr::Path(expr_path) => Some(expr_path.path.clone()),
        _ => None,
    }
}

fn read_as_ident(list: &Vec<&Expr>, index: usize) -> Option<syn::Ident> {
    let field = list.get(index)?;
   match field {
       Expr::Path(expr_ident) =>
           expr_ident.path.get_ident().cloned(),
        _ => None,
    }
}

fn read_as_string(list: &Vec<&Expr>, index: usize) -> Option<String> {
    // If we have metas, try to extract string from NameValue
    let meta = list.get(index)?;
    match meta {
        Expr::Lit(expr_lit) =>
            match &expr_lit.lit {
                syn::Lit::Str(lit_str) => Some(lit_str.value()),
                _ => None,
            },
        _ => None,
    }
}
