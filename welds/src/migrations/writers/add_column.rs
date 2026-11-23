use crate::{
    migrations::types::Type,
    migrations::writers::default_value::DefaultValue,
    writers::{ColumnWriter, TableWriter},
};
use welds_connections::Syntax;

use crate::detect::TableDef;

/// writes the SQL to Add a column on a table
pub fn write(
    syntax: Syntax,
    table: &TableDef,
    col: impl Into<String>,
    ty: &Type,
    nullable: bool,
    default_value: Option<&dyn DefaultValue>,
) -> String {
    let tablename: String = TableWriter::new(syntax).write(table.ident());
    let col: String = sanitize_column(col.into());
    let col = ColumnWriter::new(syntax).excape(&col);
    let ty_string = ty.db_type(syntax);
    let null = if nullable { "NULL" } else { "NOT NULL" };
    let coldef = format!("{ty_string} {null}");
    let mut default_def = "".to_string();
    if let Some(default) = default_value {
        default_def = default.to_raw_sql(syntax, ty);
    }

    match syntax {
        Syntax::Mssql => format!("ALTER TABLE {tablename} ADD {col} {coldef} {default_def}"),
        _ => format!("ALTER TABLE {tablename} ADD COLUMN {col} {coldef} {default_def}"),
    }
    .trim()
    .to_string()
}

/// Make sure this string is a valid column name
fn sanitize_column(input: String) -> String {
    input
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect::<String>()
}
