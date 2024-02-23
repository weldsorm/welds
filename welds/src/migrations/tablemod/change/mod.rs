use crate::{detect::TableDef, migrations::types::Type};

pub(crate) mod sqlite_writer;

pub struct Change {
    pub(super) tabledef: TableDef,
    pub(super) column_name: String,
    new_name: Option<String>,
    set_null: Option<bool>,
    new_ty: Option<Type>,
}

impl Change {
    pub(crate) fn new(tabledef: TableDef, column_name: String) -> Change {
        Change {
            tabledef,
            column_name,
            new_name: None,
            set_null: None,
            new_ty: None,
        }
    }

    pub fn rename(mut self, newname: impl Into<String>) -> Change {
        self.new_name = Some(newname.into());
        self
    }

    pub fn to_type(mut self, ty: Type) -> Change {
        self.new_ty = Some(ty);
        self
    }

    pub fn null(mut self) -> Change {
        self.set_null = Some(true);
        self
    }

    pub fn not_null(mut self) -> Change {
        self.set_null = Some(true);
        self
    }
}
