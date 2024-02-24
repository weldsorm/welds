use crate::migrations::create_table::ColumnBuilder;
use crate::migrations::create_table::IdBuilder;
use crate::migrations::create_table::TableBuilder;
use crate::migrations::types::Index;
use crate::migrations::types::Type;
use crate::model_traits::TableIdent;
use crate::Syntax;

pub fn write(table: &TableIdent, col: &ColumnBuilder) -> String {
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
