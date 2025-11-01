use syn::Ident;
use syn::Type;

#[derive(Clone)]
pub(crate) struct Column {
    pub(crate) field: Ident,
    pub(crate) selectable: bool,
    pub(crate) updateable: bool,
    pub(crate) insertable: bool,
    pub(crate) dbname: String,
    pub(crate) field_type: Type,
    pub(crate) is_option: bool,
}
