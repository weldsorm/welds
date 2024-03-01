use crate::detect::TableDef;

/// writes the SQL to Renames a column on a table
pub fn write(
    table: &TableDef,
    col: impl Into<String>,
    ty: impl Into<String>,
    nullable: bool,
) -> String {
    let tablename: String = table.ident().to_string();
    let col: String = sanitize_column(col.into());
    let ty: String = ty.into();
    let null = if nullable { "NULL" } else { "NOT NULL" };
    let coldef = format!("{ty} {null}");

    format!("ALTER TABLE {tablename} ADD COLUMN {col} {coldef}")
}

/// Make sure this string is a valid column name
fn sanitize_column(input: String) -> String {
    input
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect::<String>()
}
