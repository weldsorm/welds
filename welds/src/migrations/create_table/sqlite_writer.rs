use super::*;
use crate::migrations::types::{IdType, ToDbType, Type};
use crate::migrations::MigrationWriter;

impl MigrationWriter<sqlx::Sqlite> for TableBuilder {
    fn down_sql(&self) -> Vec<String> {
        let table = &self.ident;
        vec![format!("DROP TABLE {}", table.to_string())]
    }

    fn up_sql(&self) -> Vec<String> {
        let columns: Vec<String> = build_columns(&self.pk, &self.columns);
        let columns: String = columns.join(", ");
        let mut parts = vec![format!("CREATE TABLE {} ( {} )", self.ident, columns)];
        let index_cols = self.columns.iter().filter(|c| c.index.is_some());
        for col in index_cols {
            parts.push(build_index(&self.ident, col));
        }
        parts
    }
}

fn build_columns(idcol: &IdBuilder, cols: &[ColumnBuilder]) -> Vec<String> {
    let mut parts = Vec::default();
    parts.push(build_id_column(idcol));
    for col in cols {
        parts.push(build_column(col))
    }
    parts
}

fn build_id_column(col: &IdBuilder) -> String {
    let name = &col.name;
    let ty: String = ToDbType::<sqlx::Sqlite>::dbtype(&col.ty);
    let mut auto = "";
    if col.ty == IdType::Int || col.ty == IdType::IntSmall || col.ty == IdType::IntBig {
        auto = " AUTOINCREMENT";
    }
    format!("{name} {ty} PRIMARY KEY{auto}")
}

fn build_column(col: &ColumnBuilder) -> String {
    let name = &col.name;
    let ty: String = ToDbType::<sqlx::Sqlite>::dbtype(&col.ty);
    let null = if col.nullable { "NULL" } else { "NOT NULL" };
    format!("{name} {ty} {null}")
}

fn build_index(table: &TableIdent, col: &ColumnBuilder) -> String {
    let tablename = table.to_string();
    let colname = col.name.as_str();

    let indexname = match &col.index_name {
        Some(n) => n.to_owned(),
        None => format!("ix_{}_{}", table.name, col.name),
    };

    let index = match &col.index {
        Some(i) => i,
        None => return "".to_owned(),
    };

    let head = match index {
        Index::Unique => "CREATE UNIQUE INDEX",
        Index::Default => "CREATE INDEX",
    };
    format!("{head} {indexname} ON {tablename} ( {colname} )")
}
