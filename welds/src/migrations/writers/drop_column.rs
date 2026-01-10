use crate::Syntax;
use crate::detect::TableDef;
use crate::writers::{ColumnWriter, TableWriter};

/// writes the SQL to Renames a column on a table
pub fn write(syntax: Syntax, table: &TableDef, col: impl Into<String>) -> String {
    let tablename: String = TableWriter::new(syntax).write(&table.ident());
    let col: String = sanitize_column(col.into());
    let col = ColumnWriter::new(syntax).excape(&col);
    format!("ALTER TABLE {tablename} DROP COLUMN {col}")
}

/// Make sure this string is a valid column name
fn sanitize_column(input: String) -> String {
    input
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect::<String>()
}
