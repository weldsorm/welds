use crate::Syntax;
use crate::detect::{ColumnDef, TableDef};
mod pg_writer;
mod sqlite_writer;

/// writes the up SQL change the type/null of a column
pub fn write_up(
    syntax: Syntax,
    table: &TableDef,
    column: &ColumnDef,
    colname: impl Into<String>,
    ty: impl Into<String>,
    nullable: bool,
) -> Vec<String> {
    let tablename: String = table.ident().to_string();

    let current_col = column.name();
    let colname: String = sanitize_column(colname.into());
    let ty: String = ty.into();
    let null = if nullable { "NULL" } else { "NOT NULL" };

    match syntax {
        Syntax::Sqlite => sqlite_writer::up_sql(table, current_col, colname, ty, nullable),
        Syntax::Postgres => pg_writer::up_sql(table, column, colname, ty, nullable),
        Syntax::Mssql => vec![format!(
            "ALTER TABLE {tablename} ALTER COLUMN {colname} {ty} {null}"
        )],
        Syntax::Mysql => vec![format!(
            "ALTER TABLE {tablename} MODIFY COLUMN {colname} {ty} {null}"
        )],
    }
}

/// writes the down SQL change the type/null of a column
pub fn write_down(
    syntax: Syntax,
    table: &TableDef,
    column: &ColumnDef,
    colname: impl Into<String>,
    ty: impl Into<String>,
    nullable: bool,
) -> Vec<String> {
    let tablename: String = table.ident().to_string();

    let current_col = column.name();
    let colname: String = sanitize_column(colname.into());
    let ty: String = ty.into();
    let null = if nullable { "NULL" } else { "NOT NULL" };

    match syntax {
        Syntax::Sqlite => sqlite_writer::down_sql(table, current_col, colname, ty, nullable),
        Syntax::Postgres => pg_writer::down_sql(table, column, colname, ty, nullable),
        Syntax::Mssql => vec![format!(
            "ALTER TABLE {tablename} ALTER COLUMN {colname} {ty} {null}"
        )],
        Syntax::Mysql => vec![format!(
            "ALTER TABLE {tablename} MODIFY COLUMN {colname} {ty} {null}"
        )],
    }
}

/// Make sure this string is a valid column name
fn sanitize_column(input: String) -> String {
    input
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect::<String>()
}
