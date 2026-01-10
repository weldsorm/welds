use crate::Syntax;
use crate::model_traits::TableIdent;
use crate::writers::{ColumnWriter, TableWriter};

/// writes the SQL to Renames a column on a table
pub fn write(
    syntax: Syntax,
    table: &TableIdent,
    old_name: impl Into<String>,
    new_name: impl Into<String>,
) -> String {
    let oldname: String = sanitize_column(old_name.into());
    let oldname_esc = ColumnWriter::new(syntax).excape(&oldname);
    let newname: String = sanitize_column(new_name.into());
    let newname_esc = ColumnWriter::new(syntax).excape(&newname);
    let tablename: String = TableWriter::new(syntax).write(table);

    match syntax {
        Syntax::Mssql => format!("EXEC sp_rename '{tablename}.{oldname}', '{newname}', 'COLUMN'"),
        Syntax::Mysql => {
            format!("ALTER TABLE {tablename} RENAME COLUMN {oldname_esc} TO {newname_esc}")
        }
        _ => format!("ALTER TABLE {tablename} RENAME {oldname_esc} TO {newname_esc}"),
    }
}

/// Make sure this string is a valid column name
fn sanitize_column(input: String) -> String {
    input
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect::<String>()
}
