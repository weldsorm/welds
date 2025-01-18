use crate::migrations::create_table::ColumnBuilder;
use crate::migrations::types::Index;
use crate::migrations::types::OnDelete;
use crate::model_traits::TableIdent;
use crate::Syntax;

pub fn write(syntax: Syntax, table: &TableIdent, col: &ColumnBuilder) -> String {
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

    match index {
        Index::Unique => format!("CREATE UNIQUE INDEX {indexname} ON {tablename} ( {colname} )"),
        Index::Default => format!("CREATE INDEX {indexname} ON {tablename} ( {colname} )"),
        Index::ForeignKey((f_table, f_column, on_delete)) => {
            write_fk(syntax, table, col, f_table, f_column, *on_delete)
        }
    }
}

fn write_fk(
    syntax: Syntax,
    table: &TableIdent,
    col: &ColumnBuilder,
    foreign_table: &str,
    foreign_column: &str,
    on_delete: OnDelete,
) -> String {
    if syntax == Syntax::Sqlite {
        return String::default();
    }

    let on_delete_str = match on_delete {
        OnDelete::Cascade => "CASCADE",
        OnDelete::SetNull => "SET NULL",
        OnDelete::SetDefault => "SET DEFAULT",
        OnDelete::Restrict => "RESTRICT",
        OnDelete::NoAction => "NO ACTION",
    };

    let indexname = match &col.index_name {
        Some(n) => n.to_owned(),
        None => format!("fk_{}_{}", table.name, col.name),
    };

    let tablename = table.to_string();
    let colname = col.name.as_str();

    format!("ALTER TABLE {tablename} ADD CONSTRAINT {indexname} FOREIGN KEY ({colname}) REFERENCES {foreign_table} ({foreign_column}) ON DELETE {on_delete_str}")
}
