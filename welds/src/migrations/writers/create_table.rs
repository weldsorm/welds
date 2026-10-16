use super::create_indexes::{write as create_index, write_inline_fk};
use crate::Syntax;
use crate::detect::TableDef;
use crate::migrations::create_table::ColumnBuilder;
use crate::migrations::create_table::IdBuilder;
use crate::migrations::create_table::TableBuilder;
use crate::migrations::types::Type;
use crate::writers::ColumnWriter;
use crate::writers::TableWriter;
use crate::writers::types::pk_override;

pub fn from_def(syntax: Syntax, def: &TableDef) -> Vec<String> {
    // if we are making migrations from a tabledef must match the syntax
    // we are going to use the types from the tabledef
    assert_eq!(syntax, def.syntax());

    let mut columns: Vec<String> = Vec::default();
    let pk_columns: Vec<IdBuilder> = def
        .columns()
        .iter()
        .filter(|x| x.primary_key)
        .map(|c| {
            let pk_type = pk_override(syntax, &c.ty).unwrap_or(&c.ty);
            IdBuilder {
                name: c.name.to_string(),
                ty: Type::parse_db_type(syntax, pk_type),
            }
        })
        .collect();

    if !pk_columns.is_empty() {
        columns.extend(build_id_columns(syntax, &pk_columns));
    }

    for c in def.columns().iter().filter(|&x| !x.primary_key) {
        let col = ColumnBuilder {
            name: c.name.to_string(),
            ty: Type::parse_db_type(syntax, c.ty()),
            nullable: c.null,
            index: None,
            index_name: None,
            default: None,
        };
        columns.push(build_column(syntax, &col))
    }

    if pk_columns.len() > 1 {
        let pk_names = pk_columns
            .iter()
            .map(|col| ColumnWriter::new(syntax).excape(&col.name))
            .collect::<Vec<String>>()
            .join(", ");
        columns.push(format!("PRIMARY KEY ({pk_names})"));
    }

    let tablename = TableWriter::new(syntax).write(def.ident());

    let parts = vec![format!(
        "CREATE TABLE {} ( {} )",
        tablename,
        columns.join(", ")
    )];
    parts
}

pub fn from_builder(syntax: Syntax, tb: &TableBuilder) -> Vec<String> {
    let pks = tb.primary_keys();
    let mut columns: Vec<String> = build_columns(syntax, &pks, &tb.columns);

    // some FK indexs need to be made inline with the Table.
    let mut inline_indexes: Vec<String> = tb
        .columns
        .iter()
        .filter_map(|c| write_inline_fk(syntax, c))
        .collect();
    columns.append(&mut inline_indexes);

    let columns: String = columns.join(", ");
    let tablename = TableWriter::new(syntax).write(&tb.ident);
    let mut parts = vec![format!("CREATE TABLE {} ( {} )", tablename, columns)];
    let index_cols = tb.columns.iter().filter(|c| c.index.is_some());
    for col in index_cols {
        parts.push(create_index(syntax, &tb.ident, col));
    }
    parts
}

fn build_columns(syntax: Syntax, idcols: &[IdBuilder], cols: &[ColumnBuilder]) -> Vec<String> {
    let mut parts = build_id_columns(syntax, idcols);
    for col in cols {
        parts.push(build_column(syntax, col))
    }
    if idcols.len() > 1 {
        let pk_names = idcols
            .iter()
            .map(|col| ColumnWriter::new(syntax).excape(&col.name))
            .collect::<Vec<String>>()
            .join(", ");
        parts.push(format!("PRIMARY KEY ({pk_names})"));
    }
    parts
}

fn build_id_columns(syntax: Syntax, cols: &[IdBuilder]) -> Vec<String> {
    if cols.len() <= 1 {
        return cols.iter().map(|col| build_id_column(syntax, col, true)).collect();
    }

    cols.iter()
        .map(|col| build_id_column(syntax, col, false))
        .collect()
}

fn build_id_column(syntax: Syntax, col: &IdBuilder, include_primary_key: bool) -> String {
    let name = ColumnWriter::new(syntax).excape(&col.name);
    let ty: String = if include_primary_key {
        col.ty.db_id_type(syntax)
    } else {
        col.ty.db_type(syntax)
    };

    if !include_primary_key {
        return format!("{name} {ty} NOT NULL");
    }

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
    let name = ColumnWriter::new(syntax).excape(col.name.as_str());
    let ty: String = col.ty.db_type(syntax);

    let null = if col.nullable { "NULL" } else { "NOT NULL" };
    let mut parts = vec![name, ty, null.to_string()];

    if let Some(default) = &col.default {
        parts.push(default.to_raw_sql(syntax, &col.ty))
    }

    parts.join(" ")
}
