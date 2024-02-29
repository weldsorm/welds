//use crate::migrations::create_table::ColumnBuilder;
//use crate::migrations::create_table::IdBuilder;
//use crate::migrations::create_table::TableBuilder;
//use crate::migrations::types::Index;
//use crate::migrations::types::Type;
use crate::model_traits::TableIdent;
use crate::Syntax;

/// writes the SQL to Renames a column on a table
pub fn write(
    syntax: Syntax,
    table: &TableIdent,
    col: impl Into<String>,
    ty: impl Into<String>,
    nullable: bool,
) -> String {
    let tablename: String = table.to_string();
    let col: String = sanitize_column(col.into());
    let ty: String = ty.into();
    let null = if nullable { "NULL" } else { "NOT NULL" };

    match syntax {
        Syntax::Sqlite => todo!(),
        Syntax::Postgres => format!("ALTER TABLE {tablename} ALTER COLUMN {col} TYPE {ty} {null}"),
        Syntax::Mssql => format!("ALTER TABLE {tablename} ALTER COLUMN {col} {ty} {null}"),
        Syntax::Mysql => format!("ALTER TABLE {tablename} MODIFY COLUMN {col} {ty} {null}"),
    }
}

/// Make sure this string is a valid column name
fn sanitize_column(input: String) -> String {
    input
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect::<String>()
}
