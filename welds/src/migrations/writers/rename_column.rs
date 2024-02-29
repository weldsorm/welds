use crate::model_traits::TableIdent;
use crate::Syntax;

/// writes the SQL to Renames a column on a table
pub fn write(
    syntax: Syntax,
    table: &TableIdent,
    old_name: impl Into<String>,
    new_name: impl Into<String>,
) -> String {
    let oldname: String = sanitize_column(old_name.into());
    let newname: String = sanitize_column(new_name.into());
    let tablename: String = table.to_string();

    match syntax {
        Syntax::Mssql => format!("EXEC sp_rename '{tablename}', '{oldname}', '{newname}'"),
        _ => format!("ALTER TABLE {tablename} RENAME {oldname} TO {newname}"),
    }
}

/// Make sure this string is a valid column name
fn sanitize_column(input: String) -> String {
    input
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect::<String>()
}
