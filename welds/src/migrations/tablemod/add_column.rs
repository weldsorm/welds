use crate::Syntax;
use crate::detect::TableDef;
use crate::migrations::MigrationWriter;
use crate::migrations::types::Type;
use crate::migrations::writers::add_column;
use crate::migrations::writers::drop_column;

pub struct AddColumn {
    tabledef: TableDef,
    name: String,
    ty: Type,
    null: bool,
}

impl AddColumn {
    pub(crate) fn new(tabledef: TableDef, name: String, ty: Type) -> Self {
        Self {
            tabledef,
            name,
            null: false,
            ty,
        }
    }

    pub fn null(mut self) -> Self {
        self.null = true;
        self
    }

    pub fn not_null(mut self) -> Self {
        self.null = false;
        self
    }
}

impl MigrationWriter for AddColumn {
    fn up_sql(&self, syntax: Syntax) -> Vec<String> {
        let col = self.name.as_str();
        let ty = self.ty.db_type(syntax);
        let nullable = self.null;
        vec![add_column(syntax, &self.tabledef, col, ty, nullable)]
    }

    fn down_sql(&self, syntax: Syntax) -> Vec<String> {
        vec![drop_column(syntax, &self.tabledef, &self.name)]
    }
}
