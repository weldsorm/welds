use crate::detect::{ColumnDef, TableDef};
use crate::migrations::types::Type;
use crate::migrations::writers::alter_column_type;
use crate::migrations::writers::rename_column;
use crate::migrations::MigrationWriter;
use crate::Syntax;

//pub(crate) mod sqlite_writer;

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

    /// returns the columndef matching by its name
    fn find_column_or_unwrap(&self, name: &str) -> &ColumnDef {
        let err = format!("Could not find column '{}' in the database", name);
        self.tabledef
            .columns()
            .iter()
            .find(|&c| c.name() == self.column_name)
            .expect(&err)
    }
}

impl MigrationWriter for Change {
    fn down_sql(&self, syntax: Syntax) -> Vec<String> {
        let mut commands = Vec::default();
        let ident = self.tabledef.ident();
        let col = self.find_column_or_unwrap(&self.column_name);
        // None means no change to the null-ability of the column
        let nullable = col.null();
        let null_changed = self.set_null.is_some() && col.null() != nullable;
        // See if the new type is different from the type of the column
        let type_changed = self.new_ty.is_some()
            && self
                .new_ty
                .as_ref()
                .map(|t| t.db_type(syntax) != col.ty())
                .unwrap_or_default();
        let ty = col.ty();

        // The new name of the column after the migration
        let columnname: &str = self
            .new_name
            .as_deref()
            .unwrap_or(self.column_name.as_str());

        // If there is a change, update the column type/null
        if type_changed || null_changed {
            commands.push(alter_column_type(syntax, ident, columnname, ty, nullable));
        }

        if let Some(new_name) = &self.new_name {
            commands.push(rename_column(syntax, ident, new_name, &self.column_name));
        }
        commands
    }

    fn up_sql(&self, syntax: Syntax) -> Vec<String> {
        let mut commands = Vec::default();
        let ident = self.tabledef.ident();
        let col = self.find_column_or_unwrap(&self.column_name);
        // None means no change to the null-ability of the column
        let nullable = self.set_null.unwrap_or(col.null());
        let null_changed = self.set_null.is_some() && col.null() != nullable;

        // add the rename SQL it column was renamed
        if let Some(new_name) = &self.new_name {
            commands.push(rename_column(syntax, ident, &self.column_name, new_name));
        }

        // The name of the column from this point forward
        let columnname: &str = self
            .new_name
            .as_deref()
            .unwrap_or(self.column_name.as_str());

        // get what the type should be and if it has changed
        let ty = self
            .new_ty
            .as_ref()
            .map(|t| t.db_type(syntax))
            .unwrap_or(col.ty().to_string());

        // See if the new type is different from the type of the column
        let type_changed = self.new_ty.is_some()
            && self
                .new_ty
                .as_ref()
                .map(|t| t.db_type(syntax) != col.ty())
                .unwrap_or_default();

        // If there is a change, update the column type/null
        if type_changed || null_changed {
            commands.push(alter_column_type(syntax, ident, columnname, ty, nullable));
        }

        commands
    }
}
