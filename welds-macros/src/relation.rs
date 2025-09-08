use crate::errors::Result;
use syn::Ident;
use syn::MetaList;

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
}

impl Relation {
    pub(crate) fn new_manual(list: &MetaList) -> Result<Self> {
        let badformat = || Err(FORMAT_ERR_MANUAL.to_owned());

        let inner: Vec<_> = list.nested.iter().collect();
        if inner.len() != 4 {
            return badformat();
        }

        // read the first argument, field in relationship
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

        // Read the Rust path to the Related Model out as MetaPath
        let model = match inner[1] {
            syn::NestedMeta::Meta(m) => m,
            _ => return badformat(),
        };
        let model = match model {
            syn::Meta::Path(path) => path,
            _ => return badformat(),
        };

        // Read the value of the local field to use in the relationship
        let self_key = match inner[2] {
            syn::NestedMeta::Lit(lit) => lit,
            _ => return badformat(),
        };
        let self_key = match self_key {
            syn::Lit::Str(s) => s,
            _ => return badformat(),
        };
        let self_key = self_key.value();

        // Read the value from the other model to use for this relationship
        let foreign_key = match inner[3] {
            syn::NestedMeta::Lit(lit) => lit,
            _ => return badformat(),
        };
        let foreign_key = match foreign_key {
            syn::Lit::Str(s) => s,
            _ => return badformat(),
        };
        let foreign_key = foreign_key.value();

        let kind = Ident::new("ManualRelationship", field.span());

        Ok(Self {
            kind,
            field,
            foreign_struct: model.clone(),
            foreign_key_db: foreign_key.clone(),
            self_key_db: Some(self_key),
        })
    }

    pub(crate) fn new(list: &MetaList, kind: &'static str) -> Result<Self> {
        let badformat = || match kind {
            "BelongsTo" => Err(FORMAT_ERR_BELONGS_TO.to_owned()),
            "HasMany" => Err(FORMAT_ERR_HAS_MANY.to_owned()),
            _ => Err(FORMAT_ERR_HAS_ONE.to_owned()),
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
            foreign_key_db: foreign_key.clone(),
            self_key_db: None,
        })
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

const FORMAT_ERR_MANUAL: &str = "Invalid Format For ManualRelationship:
ManualRelationship should be in for format of
[ welds(ManualRelationship(field, struct, local_field_key, other_field_key) )]";
