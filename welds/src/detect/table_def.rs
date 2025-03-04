use crate::model_traits::TableIdent;
use crate::Syntax;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]

/// Describes what is known about a table and its relationships
pub struct TableDef {
    pub(crate) ident: TableIdent,
    pub(crate) ty: DataType,
    pub(crate) columns: Vec<ColumnDef>, // What are the columns on this table
    pub(crate) has_many: Vec<RelationDef>,
    pub(crate) has_one: Vec<RelationDef>,
    pub(crate) belongs_to: Vec<RelationDef>,
    pub(crate) belongs_to_one: Vec<RelationDef>,
    pub(crate) syntax: Syntax,
}

impl TableDef {
    pub fn ident(&self) -> &TableIdent {
        &self.ident
    }
    pub fn ty(&self) -> DataType {
        self.ty
    }
    pub fn columns(&self) -> &[ColumnDef] {
        &self.columns
    }
    pub fn has_many(&self) -> &[RelationDef] {
        &self.has_many
    }
    pub fn belongs_to(&self) -> &[RelationDef] {
        &self.belongs_to
    }
    pub fn has_one(&self) -> &[RelationDef] {
        &self.has_one
    }
    pub fn belongs_to_one(&self) -> &[RelationDef] {
        &self.belongs_to_one
    }
    pub fn syntax(&self) -> Syntax {
        self.syntax
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
/// Describes what is known about a table
pub struct TableDefSingle {
    pub(crate) ident: TableIdent,
    pub(crate) ty: DataType,
    pub(crate) columns: Vec<ColumnDef>, // What are the columns on this table
    pub(crate) syntax: Syntax,
}

impl From<TableDef> for TableDefSingle {
    fn from(t: TableDef) -> Self {
        Self {
            ident: t.ident,
            ty: t.ty,
            columns: t.columns,
            syntax: t.syntax,
        }
    }
}

impl TableDefSingle {
    pub fn ident(&self) -> &TableIdent {
        &self.ident
    }
    pub fn ty(&self) -> DataType {
        self.ty
    }
    pub fn columns(&self) -> &[ColumnDef] {
        &self.columns
    }
    pub fn syntax(&self) -> Syntax {
        self.syntax
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ColumnDef {
    pub(crate) name: String,
    pub(crate) ty: String,
    pub(crate) null: bool,
    pub(crate) primary_key: bool,
    pub(crate) updatable: bool,
}

impl ColumnDef {
    /// return the name of this column
    pub fn name(&self) -> &str {
        &self.name
    }

    /// returns the database type of this column
    pub fn ty(&self) -> &str {
        &self.ty
    }

    /// return if this column can hold null values
    pub fn null(&self) -> bool {
        self.null
    }

    /// returns true if this column holds a primary key
    pub fn primary_key(&self) -> bool {
        self.primary_key
    }
    /// returns true is this column can be written to. I.E. not readonly
    pub fn updatable(&self) -> bool {
        self.updatable
    }

    /// returns a model_traits::Column. This can be used for queries.
    pub fn as_query_column(&self, syntax: Syntax) -> Option<crate::model_traits::Column> {
        let db_type = self.ty();
        let rust_type_pair = crate::writers::types::recommended_rust_type(syntax, db_type)?;
        let rust_type = rust_type_pair.rust_type();
        Some(crate::model_traits::Column::new(
            &self.name,
            rust_type,
            self.null(),
        ))
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum DataType {
    Table,
    View,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct RelationDef {
    other_table: TableIdent,
    foreign_key: String,
    primary_key: String,
}

impl RelationDef {
    pub(crate) fn new(ident: TableIdent, foreign_key: &str, primary_key: &str) -> Self {
        Self {
            other_table: ident,
            foreign_key: foreign_key.to_owned(),
            primary_key: primary_key.to_owned(),
        }
    }

    /// The Other side of this relationship
    pub fn other_table(&self) -> &TableIdent {
        &self.other_table
    }
    /// this is the foreign_key side column regardless of which side this defines
    pub fn foreign_key(&self) -> &str {
        &self.foreign_key
    }
    /// this is the column the fk point to, regardless of which side this defines
    pub fn primary_key(&self) -> &str {
        &self.primary_key
    }
}

#[cfg(feature = "mock")]
/// This module allows you to mock TableDef
/// useful while testing
pub mod mock {
    use super::*;

    pub struct MockColumnDef(ColumnDef);
    impl MockColumnDef {
        pub fn new(name: impl Into<String>, ty: impl Into<String>) -> MockColumnDef {
            let name: String = name.into();
            let ty: String = ty.into();
            MockColumnDef(ColumnDef {
                name,
                ty,
                null: false,
                primary_key: false,
                updatable: true,
            })
        }

        pub fn null(mut self) -> Self {
            self.0.null = true;
            self
        }

        pub fn primary_key(mut self) -> Self {
            self.0.primary_key = true;
            self
        }
        pub fn readonly(mut self) -> Self {
            self.0.updatable = false;
            self
        }

        pub fn build(self) -> ColumnDef {
            self.0
        }
    }

    pub struct MockTableDef(TableDef);

    impl MockTableDef {
        pub fn new(syntax: Syntax, name: impl Into<String>) -> MockTableDef {
            let name: String = name.into();
            let ident = TableIdent::parse(&name);
            MockTableDef(TableDef {
                syntax,
                ident,
                ty: DataType::Table,
                columns: Vec::default(),
                has_many: Vec::default(),
                belongs_to: Vec::default(),
                has_one: Vec::default(),
                belongs_to_one: Vec::default(),
            })
        }

        pub fn as_view(mut self) -> Self {
            self.0.ty = DataType::View;
            self
        }

        pub fn as_table(mut self) -> Self {
            self.0.ty = DataType::Table;
            self
        }

        pub fn with_pk(mut self, name: impl Into<String>, ty: impl Into<String>) -> Self {
            self.0.columns.push(ColumnDef {
                name: name.into(),
                ty: ty.into(),
                null: false,
                primary_key: true,
                updatable: true,
            });
            self
        }

        pub fn with_column(mut self, name: impl Into<String>, ty: impl Into<String>) -> Self {
            self.0.columns.push(ColumnDef {
                name: name.into(),
                ty: ty.into(),
                null: false,
                primary_key: false,
                updatable: true,
            });
            self
        }

        pub fn with_nullable_column(
            mut self,
            name: impl Into<String>,
            ty: impl Into<String>,
        ) -> Self {
            self.0.columns.push(ColumnDef {
                name: name.into(),
                ty: ty.into(),
                null: true,
                primary_key: false,
                updatable: true,
            });
            self
        }

        pub fn build(self) -> TableDef {
            self.0
        }
    }
}
