use crate::table::TableIdent;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]

/// Describes what is known about a table and its relationships
pub struct TableDef {
    pub(crate) ident: TableIdent,
    pub(crate) ty: DataType,
    pub(crate) columns: Vec<ColumnDef>, // What are the columns on this table
    pub(crate) has_many: Vec<RelationDef>,
    pub(crate) belongs_to: Vec<RelationDef>,
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
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
/// Describes what is known about a table
pub struct TableDefSingle {
    pub(crate) ident: TableIdent,
    pub(crate) ty: DataType,
    pub(crate) columns: Vec<ColumnDef>, // What are the columns on this table
}

impl From<TableDef> for TableDefSingle {
    fn from(t: TableDef) -> Self {
        Self {
            ident: t.ident,
            ty: t.ty,
            columns: t.columns,
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
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ColumnDef {
    pub name: String,
    pub ty: String,
    pub null: bool,
    pub primary_key: bool,
    pub updatable: bool,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum DataType {
    Table,
    View,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct RelationDef {
    /// The Other side of this relationship
    pub other_table: TableIdent,
    /// this is the foreign_key side column regardless of which side this defines
    pub foreign_key: String,
    /// this is the column the fk point to, regardless of which side this defines
    pub primary_key: String,
}

impl RelationDef {
    pub(crate) fn new(ident: TableIdent, foreign_key: &str, primary_key: &str) -> Self {
        Self {
            other_table: ident,
            foreign_key: foreign_key.to_owned(),
            primary_key: primary_key.to_owned(),
        }
    }
}
