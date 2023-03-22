use crate::attributes;
use crate::column::Column;
use syn::Ident;

pub(crate) struct Info {
    pub defstruct: Ident,
    pub schemastruct: Ident,
    pub engines_ident: Vec<Ident>,
    pub columns: Vec<Column>,
    pub pks: Vec<Column>,
    pub tablename: String,
    pub schemaname: Option<String>,
    pub readonly: bool,
}

impl Info {
    pub fn new(ast: &syn::DeriveInput) -> Self {
        let engines = attributes::get_engines(&ast);
        let defstruct = attributes::get_scructname(&ast);
        let schemastruct_name = format!("{}Schema", defstruct);
        let schemastruct = Ident::new(&schemastruct_name, defstruct.span());
        let tablename = attributes::get_tablename(&ast);
        let schemaname = attributes::get_schemaname(&ast);
        let columns = attributes::get_columns(&ast);
        let pks = attributes::get_pks(&ast);
        let readonly = attributes::get_readonly(&ast);

        let engines_ident = engines
            .iter()
            .map(|e| Ident::new(e.as_str(), defstruct.span()))
            .collect();

        Self {
            //engines,
            engines_ident,
            columns,
            pks,
            defstruct,
            schemastruct,
            tablename,
            schemaname,
            readonly,
        }
    }
}
