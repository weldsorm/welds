use super::Relation;
use super::{read_as_ident, read_as_path, read_as_string};
use crate::errors::Result;
use syn::Ident;
use syn::MetaList;

impl Relation {
    pub(crate) fn basic(list: &MetaList) -> Result<Vec<Self>> {
        let kind = list.path.get_ident().unwrap().to_string();
        let kind_str: &str = kind.as_str();

        let badformat = || match kind_str {
            "BelongsTo" => FORMAT_ERR_BELONGS_TO.to_owned(),
            "HasMany" => FORMAT_ERR_HAS_MANY.to_owned(),
            "HasOne" => FORMAT_ERR_HAS_ONE.to_owned(),
            _ => panic!("Unknown relationship type ({})", kind_str),
        };

        let inner: Vec<_> = list.nested.iter().collect();
        if inner.len() != 3 {
            return Err(badformat());
        }

        let field = read_as_ident(list, 0).ok_or_else(badformat)?;
        let model = read_as_path(list, 1).ok_or_else(badformat)?;
        let foreign_key = read_as_string(list, 2).ok_or_else(badformat)?;

        let kind = Ident::new(kind.as_str(), field.span());

        Ok(vec![Self {
            kind,
            field,
            foreign_struct: model.clone(),
            foreign_key_db: foreign_key.clone(),
            self_key_db: None,
            is_jointable: false,
        }])
    }
}

const FORMAT_ERR_HAS_MANY: &str = "Invalid Format For HasMany:
HasMany should be in for format of
[ welds(HasMany(field, struct, foreign_key_str) )]";

const FORMAT_ERR_HAS_ONE: &str = "Invalid Format For HasOne:
HasOne should be in for format of
[ welds(HasOne(field, struct, foreign_key_str) )]";

const FORMAT_ERR_BELONGS_TO: &str = "Invalid Format For BelongsTo:
BelongsTo should be in for format of
[ welds(BelongsTo(field, struct, foreign_key_str) )]";
