use crate::attributes;
use crate::column::Column;
use crate::errors::Result;
use crate::relation::Relation;
use syn::Ident;

pub(crate) struct Info {
    pub defstruct: Ident,
    pub schemastruct: Ident,
    pub engines_ident: Vec<Ident>,
    pub columns: Vec<Column>,
    pub pks: Vec<Column>,
    pub relations: Vec<Relation>,
    pub relations_struct: Ident,
    pub tablename: String,
    pub schemaname: Option<String>,
    pub readonly: bool,
}

impl Info {
    pub fn new(ast: &syn::DeriveInput) -> Result<Self> {
        let engines = attributes::get_engines(ast);
        let relations = attributes::get_relations(ast)?;
        let defstruct = attributes::get_scructname(ast);
        let schemastruct_name = format!("{}Schema", defstruct);
        let schemastruct = Ident::new(&schemastruct_name, defstruct.span());
        let relations_struct_name = format!("{}Relation", defstruct);
        let relations_struct = Ident::new(&relations_struct_name, defstruct.span());
        let tablename = attributes::get_tablename(ast);
        let schemaname = attributes::get_schemaname(ast);
        let columns = attributes::get_columns(ast);
        let pks = attributes::get_pks(ast);
        let readonly = attributes::get_readonly(ast);

        let engines_ident = engines
            .iter()
            .map(|e| Ident::new(e.as_str(), defstruct.span()))
            .collect();

        Ok(Self {
            //engines,
            engines_ident,
            columns,
            pks,
            defstruct,
            relations,
            schemastruct,
            relations_struct,
            tablename,
            schemaname,
            readonly,
        })
    }
}
