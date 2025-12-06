use super::{read_as_path, Relation};
use super::{read_as_ident, read_as_string};
use crate::errors::Result;
use syn::{Expr, Ident};
use syn::MetaList;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::token::Comma;

impl Relation {
    pub(crate) fn new_manual(list: &MetaList) -> Result<Vec<Self>> {
        let badformat = || FORMAT_ERR_MANUAL.to_owned();

        let list =  Punctuated::<Expr, Comma>::parse_terminated.parse2(list.tokens.clone())
            .map_err(|_|badformat())?;
        let list: &Vec<_> = &list.iter().collect();

        if list.len() != 4 {
            return Err(badformat());
        }

        let field = read_as_ident(list, 0).ok_or_else(badformat)?;
        let model = read_as_path(list, 1).ok_or_else(badformat)?;
        let self_key = read_as_string(list, 2).ok_or_else(badformat)?;
        let foreign_key = read_as_string(list, 3).ok_or_else(badformat)?;

        let kind = Ident::new("ManualRelationship", field.span());

        Ok(vec![Self {
            kind,
            field,
            foreign_struct: model.clone(),
            foreign_key_db: foreign_key.clone(),
            self_key_db: Some(self_key),
            is_jointable: false,
        }])
    }
}

const FORMAT_ERR_MANUAL: &str = "Invalid Format For ManualRelationship:
ManualRelationship should be in for format of
[ welds(ManualRelationship(field, struct, local_field_key, other_field_key) )]";
