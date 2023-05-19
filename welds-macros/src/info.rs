use crate::attributes;
use crate::column::Column;
use crate::engine::Engine;
use crate::errors::Result;
use crate::relation::Relation;
use syn::Ident;

pub(crate) struct Info {
    pub defstruct: Ident,
    pub schemastruct: Ident,
    pub engines_path: Vec<syn::Path>,
    pub columns: Vec<Column>,
    pub pks: Vec<Column>,
    pub relations: Vec<Relation>,
    pub relations_struct: Ident,
    pub tablename: String,
    pub schemaname: Option<String>,
    pub readonly: bool,
    pub welds_path: syn::Path,
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
        let welds_path = attributes::get_welds_path(ast);

        let engines_path = engines
            .iter()
            .map(|eng| build_engine_path(eng, &welds_path))
            .collect();

        Ok(Self {
            engines_path,
            columns,
            pks,
            defstruct,
            relations,
            schemastruct,
            relations_struct,
            tablename,
            schemaname,
            readonly,
            welds_path,
        })
    }
}

fn build_engine_path(engine: &Engine, wp: &syn::Path) -> syn::Path {
    use quote::quote;

    let q = match engine {
        Engine::Postgres => quote!(sqlx::Postgres),
        Engine::Mssql => quote!(#wp::Mssql),
        Engine::Mysql => quote!(sqlx::MySql),
        Engine::Sqlite => quote!(sqlx::Sqlite),
    };

    syn::parse(q.into()).unwrap()
}
