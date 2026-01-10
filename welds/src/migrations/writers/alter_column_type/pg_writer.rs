use super::*;
use crate::Syntax;
use crate::writers::TableWriter;

pub(crate) fn up_sql(
    syntax: Syntax,
    table: &TableDef,
    column: &ColumnDef,
    colname: String,
    ty: String,
    nullable: bool,
) -> Vec<String> {
    let mut cmds = Vec::default();
    let tablename: String = TableWriter::new(syntax).write(&table.ident());
    let colname = ColumnWriter::new(syntax).excape(&colname);

    // Change the type
    if column.ty() != ty {
        let s = format!("ALTER TABLE {tablename} ALTER COLUMN {colname} TYPE {ty}");
        cmds.push(s);
    }

    // change to NULL
    if column.null() != nullable && nullable {
        let s = format!("ALTER TABLE {tablename} ALTER COLUMN {colname} DROP NOT NULL");
        cmds.push(s);
    }

    // change to NOT NULL
    if column.null() != nullable && !nullable {
        let s = format!("ALTER TABLE {tablename} ALTER COLUMN {colname} SET NOT NULL");
        cmds.push(s);
    }

    cmds
}

pub(crate) fn down_sql(
    syntax: Syntax,
    table: &TableDef,
    column: &ColumnDef,
    colname: String,
    _ty: String,
    _nullable: bool,
) -> Vec<String> {
    let mut cmds = Vec::default();
    let tablename: String = TableWriter::new(syntax).write(&table.ident());
    let colname = ColumnWriter::new(syntax).excape(&colname);

    // NOTE: changing the type to the type it currently is is valid in PG
    // IE: table has type TEXT for column name, and changing it to type TEXT

    // Change the type
    let og_ty = column.ty();
    let s = format!("ALTER TABLE {tablename} ALTER COLUMN {colname} TYPE {og_ty}");
    cmds.push(s);

    // change to NULL
    if column.null() {
        let s = format!("ALTER TABLE {tablename} ALTER COLUMN {colname} DROP NOT NULL");
        cmds.push(s);
    }

    // change to NOT NULL
    if !column.null() {
        let s = format!("ALTER TABLE {tablename} ALTER COLUMN {colname} SET NOT NULL");
        cmds.push(s);
    }

    cmds
}
