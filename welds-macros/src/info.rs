use crate::attributes;
use crate::column::Column;
use crate::errors::Result;
use crate::hook::Hook;
use crate::relation::Relation;
use syn::Ident;

pub(crate) struct Info {
    pub defstruct: Ident,
    pub schemastruct: Ident,
    pub colstruct: Ident,
    pub columns: Vec<Column>,
    pub pks: Vec<Column>,
    pub relations: Vec<Relation>,
    pub hooks: Vec<Hook>,
    pub relations_struct: Ident,
    pub tablename: String,
    pub schemaname: Option<String>,
    pub readonly: bool,
    pub welds_path: syn::Path,
}

impl Info {
    pub fn new(ast: &syn::DeriveInput) -> Result<Self> {
        let relations = attributes::get_relations(ast)?;
        let hooks = attributes::get_hooks(ast)?;
        let defstruct = attributes::get_scructname(ast);
        let schemastruct_name = format!("{}Schema", defstruct);
        let schemastruct = Ident::new(&schemastruct_name, defstruct.span());
        let colstruct_name = format!("{}Columns", defstruct);
        let colstruct = Ident::new(&colstruct_name, defstruct.span());
        let relations_struct_name = format!("{}Relation", defstruct);
        let relations_struct = Ident::new(&relations_struct_name, defstruct.span());
        let tablename = attributes::get_tablename(ast);
        let schemaname = attributes::get_schemaname(ast);
        let columns = attributes::get_columns(ast);
        let pks = attributes::get_pks(ast);
        let readonly = attributes::get_readonly(ast);
        let welds_path = attributes::get_welds_path(ast);

        Ok(Self {
            columns,
            pks,
            defstruct,
            relations,
            hooks,
            schemastruct,
            colstruct,
            relations_struct,
            tablename,
            schemaname,
            readonly,
            welds_path,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::Info;
    use crate::column::Column;
    use proc_macro2::{Ident, Span};
    //use syn::{parse_quote, parse_str, Path, Type};

    impl Info {
        pub(crate) fn mock() -> Info {
            Info {
                defstruct: Ident::new("Mock", Span::call_site()),
                schemastruct: Ident::new("MockSchema", Span::call_site()),
                colstruct: Ident::new("MockColumns", Span::call_site()),
                columns: Vec::default(),
                pks: Vec::default(),
                relations: Vec::default(),
                hooks: Vec::default(),
                relations_struct: Ident::new("MockRelationships", Span::call_site()),
                tablename: "datables".to_string(),
                schemaname: Some("daschema".to_string()),
                readonly: false,
                welds_path: Ident::new("welds", Span::call_site()).into(),
            }
        }

        pub(crate) fn add_column(
            mut self,
            name: impl Into<String>,
            ty: impl Into<String>,
            null: bool,
        ) -> Info {
            let name: String = name.into();
            let field: Ident = Ident::new(&name, Span::call_site());
            let ty: String = ty.into();
            let field_type: syn::Type = syn::parse_str(&ty).unwrap();
            let col = Column {
                field,
                ignore: false,
                dbname: name,
                field_type,
                is_option: null,
            };
            self.columns.push(col);
            self
        }

        pub(crate) fn add_pk(mut self, name: impl Into<String>, ty: impl Into<String>) -> Info {
            let name: String = name.into();
            let field: Ident = Ident::new(&name, Span::call_site());
            let ty: String = ty.into();
            let field_type: syn::Type = syn::parse_str(&ty).unwrap();

            let col = Column {
                field,
                ignore: false,
                dbname: name,
                field_type,
                is_option: false,
            };
            self.columns.push(col.clone());
            self.pks.push(col);
            self
        }
    }
}
