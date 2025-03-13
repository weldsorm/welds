use syn::Ident;
use syn::Type;

#[derive(Clone)]
pub(crate) struct Column {
    pub(crate) field: Ident,
    pub(crate) ignore: bool,
    pub(crate) dbname: String,
    pub(crate) field_type: Type,
    pub(crate) full_field_type: Type,
    pub(crate) is_option: bool,
}

impl Column {
    // returns the full type of the column
    // including the wrapping Option.
    pub(crate) fn full_type(&self) -> Type {
        self.full_field_type.clone()
    }
}
