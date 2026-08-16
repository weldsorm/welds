use crate::Syntax;
use crate::detect::TableDef;
use crate::migrations::MigrationWriter;
use crate::migrations::types::Type;
use crate::migrations::writers::add_column;
use crate::migrations::writers::default_value::DefaultValue;
use crate::migrations::writers::drop_column;

pub struct AddColumn {
    tabledef: Option<TableDef>,
    name: String,
    ty: Type,
    null: bool,
    default_value: Option<Box<dyn DefaultValue>>,
}

impl AddColumn {
    pub(crate) fn new(tabledef: Option<TableDef>, name: String, ty: Type) -> Self {
        Self {
            tabledef,
            name,
            null: false,
            ty,
            default_value: None,
        }
    }

    /// Sets the column allow NULL values
    pub fn null(mut self) -> Self {
        self.null = true;
        self
    }

    /// Sets the column prevent NULL values
    pub fn not_null(mut self) -> Self {
        self.null = false;
        self
    }

    pub fn with_default<T>(mut self, default_value: T) -> Self
    where
        T: DefaultValue + 'static,
    {
        self.default_value = Some(Box::new(default_value));
        self
    }
}

impl MigrationWriter for AddColumn {
    fn up_sql(&self, syntax: Syntax) -> Vec<String> {
        let tabledef = match &self.tabledef {
            Some(x) => x,
            None => return vec![],
        };
        let col = self.name.as_str();
        let nullable = self.null;
        let default = self.default_value.as_deref();
        vec![add_column(
            syntax, tabledef, col, &self.ty, nullable, default,
        )]
    }

    fn down_sql(&self, syntax: Syntax) -> Vec<String> {
        let tabledef = match &self.tabledef {
            Some(x) => x,
            None => return vec![],
        };
        vec![drop_column(syntax, tabledef, &self.name)]
    }
}
