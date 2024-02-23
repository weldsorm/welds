use super::types::{IdType, Index, Type};
use crate::table::TableIdent;

mod sqlite_writer;

pub fn create_table(name: impl Into<String>) -> TableBuilder {
    let name: String = name.into();
    let ident = TableIdent::parse(&name);
    TableBuilder {
        ident,
        pk: IdBuilder::default(),
        columns: Vec::default(),
    }
}

pub struct TableBuilder {
    ident: TableIdent,
    pk: IdBuilder,
    columns: Vec<ColumnBuilder>,
}

impl TableBuilder {
    pub fn id(mut self, lam: fn(fn(&str, IdType) -> IdBuilder) -> IdBuilder) -> Self {
        let builder = |name: &str, ty: IdType| -> IdBuilder {
            IdBuilder {
                name: name.to_string(),
                ty,
            }
        };
        let col = lam(builder);
        self.pk = col;
        self
    }

    pub fn column(mut self, lam: fn(fn(&str, Type) -> ColumnBuilder) -> ColumnBuilder) -> Self {
        let builder = |name: &str, ty: Type| -> ColumnBuilder {
            ColumnBuilder {
                name: name.to_string(),
                ty,
                nullable: false,
                index: None,
                index_name: None,
            }
        };
        let col = lam(builder);
        self.columns.push(col);
        self
    }
}

pub struct ColumnBuilder {
    name: String,
    ty: Type,
    nullable: bool,
    index: Option<Index>,
    index_name: Option<String>,
}

impl ColumnBuilder {
    pub fn is_null(mut self) -> Self {
        self.nullable = true;
        self
    }

    pub fn with_index_name(mut self, name: impl Into<String>) -> Self {
        let name: String = name.into();
        self.index_name = Some(name);
        if self.index.is_none() {
            self.index = Some(Index::Default);
        }
        self
    }

    pub fn create_index(mut self) -> Self {
        self.index = Some(Index::Default);
        self
    }

    pub fn create_unique_index(mut self) -> Self {
        self.index = Some(Index::Unique);
        self
    }
}

pub struct IdBuilder {
    name: String,
    ty: IdType,
}

impl Default for IdBuilder {
    fn default() -> Self {
        Self {
            name: "id".to_string(),
            ty: IdType::Int,
        }
    }
}
