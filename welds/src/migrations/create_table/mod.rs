use super::types::{Index, OnDelete, Type};
use crate::migrations::writers;
use crate::migrations::MigrationWriter;
use crate::model_traits::TableIdent;
use crate::Syntax;

impl MigrationWriter for TableBuilder {
    fn down_sql(&self, _syntax: Syntax) -> Vec<String> {
        let table = &self.ident;
        vec![writers::drop_table(table)]
    }

    fn up_sql(&self, syntax: Syntax) -> Vec<String> {
        writers::create_table::from_builder(syntax, self)
    }
}

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
    pub(crate) ident: TableIdent,
    pub(crate) pk: IdBuilder,
    pub(crate) columns: Vec<ColumnBuilder>,
}

type ColumnLambda = fn(&str, Type) -> ColumnBuilder;
type TableLambda = fn(&str, Type) -> IdBuilder;

impl TableBuilder {
    pub fn id(mut self, lam: fn(TableLambda) -> IdBuilder) -> Self {
        let builder = |name: &str, ty: Type| -> IdBuilder {
            IdBuilder {
                name: name.to_string(),
                ty,
            }
        };
        let col = lam(builder);
        self.pk = col;
        self
    }

    pub fn column(mut self, lam: fn(ColumnLambda) -> ColumnBuilder) -> Self {
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
    pub(crate) name: String,
    pub(crate) ty: Type,
    pub(crate) nullable: bool,
    pub(crate) index: Option<Index>,
    pub(crate) index_name: Option<String>,
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

    pub fn create_foreign_key(
        mut self,
        table: impl Into<String>,
        column: impl Into<String>,
        on_delete: OnDelete,
    ) -> Self {
        self.index = Some(Index::ForeignKey((table.into(), column.into(), on_delete)));
        self
    }
}

pub struct IdBuilder {
    pub(crate) name: String,
    pub(crate) ty: Type,
}

impl Default for IdBuilder {
    fn default() -> Self {
        Self {
            name: "id".to_string(),
            ty: Type::Int,
        }
    }
}

#[cfg(test)]
mod tests;
