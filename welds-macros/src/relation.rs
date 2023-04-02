use crate::errors::Result;
use syn::Ident;
use syn::MetaList;

#[derive(Debug)]
pub(crate) struct Relation {
    pub(crate) kind: Ident,
    pub(crate) field: Ident,
    pub(crate) foreign_struct: syn::Path,
    pub(crate) foreign_key: String,
}

impl Relation {
    pub(crate) fn new(list: &MetaList, kind: &'static str) -> Result<Self> {
        let badformat = || {
            if kind == "BelongsTo" {
                return Err(FORMAT_ERR_BELONGS_TO.to_owned());
            } else {
                return Err(FORMAT_ERR_HAS_MANY.to_owned());
            }
        };

        let inner: Vec<_> = list.nested.iter().collect();
        if inner.len() != 3 {
            return badformat();
        }

        let field = match inner[0] {
            syn::NestedMeta::Meta(m) => m,
            _ => return badformat(),
        };
        let field = match field {
            syn::Meta::Path(path) => path,
            _ => return badformat(),
        };
        if field.segments.len() != 1 {
            return badformat();
        }
        let field = field.segments[0].ident.clone();

        let model = match inner[1] {
            syn::NestedMeta::Meta(m) => m,
            _ => return badformat(),
        };
        let model = match model {
            syn::Meta::Path(path) => path,
            _ => return badformat(),
        };

        let foreign_key = match inner[2] {
            syn::NestedMeta::Lit(lit) => lit,
            _ => return badformat(),
        };
        let foreign_key = match foreign_key {
            syn::Lit::Str(s) => s,
            _ => return badformat(),
        };
        let foreign_key = foreign_key.value();

        let kind = Ident::new(kind, field.span());

        Ok(Self {
            kind,
            field,
            foreign_struct: model.clone(),
            foreign_key,
        })
    }
}

const FORMAT_ERR_HAS_MANY: &'static str = "Invalid Format For HasMany:
HasMany should be in for format of
[ welds(HasMany(field, struct, foreign_key_str) )]";

const FORMAT_ERR_BELONGS_TO: &'static str = "Invalid Format For BelongsTo:
BelongsTo should be in for format of
[ welds(BelongsTo(field, struct, foreign_key_str) )]";
