use crate::Syntax;
use crate::detect::{ColumnDef, TableDef};
mod pg_writer;
mod sqlite_writer;
use crate::writers::{ColumnWriter, TableWriter};

/// writes the up SQL change the type/null of a column
pub fn write_up(
    syntax: Syntax,
    table: &TableDef,
    column: &ColumnDef,
    colname: impl Into<String>,
    ty: impl Into<String>,
    nullable: bool,
) -> Vec<String> {
    let tablename: String = TableWriter::new(syntax).write(&table.ident());

    let current_col = column.name();
    let colname: String = sanitize_column(colname.into());
    let colname_esc = ColumnWriter::new(syntax).excape(&colname);
    let ty: String = ty.into();
    let null = if nullable { "NULL" } else { "NOT NULL" };

    match syntax {
        Syntax::Sqlite => sqlite_writer::up_sql(syntax, table, current_col, colname, ty, nullable),
        Syntax::Postgres => pg_writer::up_sql(syntax, table, column, colname, ty, nullable),
        Syntax::Mssql => vec![format!(
            "ALTER TABLE {tablename} ALTER COLUMN {colname_esc} {ty} {null}"
        )],
        Syntax::Mysql => vec![format!(
            "ALTER TABLE {tablename} MODIFY COLUMN {colname_esc} {ty} {null}"
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
    let tablename: String = TableWriter::new(syntax).write(&table.ident());

    let current_col = column.name();
    let colname: String = sanitize_column(colname.into());
    let colname_esc = ColumnWriter::new(syntax).excape(&colname);
    let ty: String = ty.into();
    let null = if nullable { "NULL" } else { "NOT NULL" };

    match syntax {
        Syntax::Sqlite => {
            sqlite_writer::down_sql(syntax, table, current_col, colname, ty, nullable)
        }
        Syntax::Postgres => pg_writer::down_sql(syntax, table, column, colname, ty, nullable),
        Syntax::Mssql => vec![format!(
            "ALTER TABLE {tablename} ALTER COLUMN {colname_esc} {ty} {null}"
        )],
        Syntax::Mysql => vec![format!(
            "ALTER TABLE {tablename} MODIFY COLUMN {colname_esc} {ty} {null}"
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
