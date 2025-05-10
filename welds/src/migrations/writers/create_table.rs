use super::create_index;
use crate::Syntax;
use crate::detect::TableDef;
use crate::migrations::create_table::ColumnBuilder;
use crate::migrations::create_table::IdBuilder;
use crate::migrations::create_table::TableBuilder;
use crate::migrations::types::Type;
use crate::writers::TableWriter;
use crate::writers::types::pk_override;

pub fn from_def(syntax: Syntax, def: &TableDef) -> Vec<String> {
    // if we are making migrations from a tabledef must match the syntax
    // we are going to use the types from the tabledef
    assert_eq!(syntax, def.syntax());

    let mut columns: Vec<String> = Vec::default();

    for c in def.columns().iter().filter(|x| x.primary_key) {
        let pk_type = pk_override(syntax, &c.ty).unwrap_or(&c.ty);
        let col = IdBuilder {
            name: c.name.to_string(),
            ty: Type::parse_db_type(syntax, pk_type),
        };
        columns.push(build_id_column(syntax, &col))
    }

    for c in def.columns().iter().filter(|&x| !x.primary_key) {
        let col = ColumnBuilder {
            name: c.name.to_string(),
            ty: Type::parse_db_type(syntax, c.ty()),
            nullable: c.null,
            index: None,
            index_name: None,
        };
        columns.push(build_column(syntax, &col))
    }

    let tablename = TableWriter::new(syntax).write(&def.ident());

    let parts = vec![format!(
        "CREATE TABLE {} ( {} )",
        tablename,
        columns.join(", ")
    )];
    parts
}

pub fn from_builder(syntax: Syntax, tb: &TableBuilder) -> Vec<String> {
    let columns: Vec<String> = build_columns(syntax, &tb.pk, &tb.columns);
    let columns: String = columns.join(", ");
    let tablename = TableWriter::new(syntax).write(&tb.ident);
    let mut parts = vec![format!("CREATE TABLE {} ( {} )", tablename, columns)];
    let index_cols = tb.columns.iter().filter(|c| c.index.is_some());
    for col in index_cols {
        parts.push(create_index(syntax, &tb.ident, col));
    }
    parts
}

fn build_columns(syntax: Syntax, idcol: &IdBuilder, cols: &[ColumnBuilder]) -> Vec<String> {
    let mut parts = Vec::default();
    parts.push(build_id_column(syntax, idcol));
    for col in cols {
        parts.push(build_column(syntax, col))
    }
    parts
}

fn build_id_column(syntax: Syntax, col: &IdBuilder) -> String {
    let name = &col.name;
    let ty: String = col.ty.db_id_type(syntax);
    let mut tail = "PRIMARY KEY";
    if col.ty == Type::Int || col.ty == Type::IntSmall || col.ty == Type::IntBig {
        tail = match syntax {
            Syntax::Mssql => "IDENTITY(1,1) PRIMARY KEY",
            Syntax::Mysql => "AUTO_INCREMENT PRIMARY KEY",
            Syntax::Postgres => "PRIMARY KEY",
            Syntax::Sqlite => "PRIMARY KEY AUTOINCREMENT",
        }
    }
    format!("{name} {ty} {tail} NOT NULL")
}

fn build_column(syntax: Syntax, col: &ColumnBuilder) -> String {
    let name = &col.name;
    let ty: String = col.ty.db_type(syntax);

    let null = if col.nullable { "NULL" } else { "NOT NULL" };
    format!("{name} {ty} {null}")
}
