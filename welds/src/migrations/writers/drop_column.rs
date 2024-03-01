use crate::detect::TableDef;

/// writes the SQL to Renames a column on a table
pub fn write(table: &TableDef, col: impl Into<String>) -> String {
    let tablename: String = table.ident().to_string();
    let col: String = sanitize_column(col.into());
    format!("ALTER TABLE {tablename} DROP COLUMN {col}")
}

/// Make sure this string is a valid column name
fn sanitize_column(input: String) -> String {
    input
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect::<String>()
}
