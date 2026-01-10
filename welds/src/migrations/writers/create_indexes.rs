use crate::Syntax;
use crate::migrations::create_table::ColumnBuilder;
use crate::migrations::types::Index;
use crate::migrations::types::OnDelete;
use crate::model_traits::TableIdent;
use crate::writers::ColumnWriter;
use crate::writers::TableWriter;

pub fn write(syntax: Syntax, table: &TableIdent, col: &ColumnBuilder) -> String {
    let tablename = TableWriter::new(syntax).write(table);
    let colname = ColumnWriter::new(syntax).excape(col.name.as_str());

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

/// write the fk crateion for sqlite. inline while table is being created
pub(crate) fn write_inline_fk(syntax: Syntax, col: &ColumnBuilder) -> Option<String> {
    // only sqlite needs FKs create in the table create
    if syntax != Syntax::Sqlite {
        return None;
    }

    // pull out the info about the FK to create
    let colname = ColumnWriter::new(syntax).excape(col.name.as_str());
    let (f_table, f_column, on_delete) = match col.index.as_ref()? {
        Index::ForeignKey(fk_args) => fk_args,
        _ => return None,
    };
    let f_column = ColumnWriter::new(syntax).excape(f_column.as_str());

    let on_delete_str = match on_delete {
        OnDelete::Cascade => "CASCADE",
        OnDelete::SetNull => "SET NULL",
        OnDelete::SetDefault => "SET DEFAULT",
        OnDelete::Restrict => "RESTRICT",
        OnDelete::NoAction => "NO ACTION",
    };

    Some(format!(
        "FOREIGN KEY ({colname}) REFERENCES {f_table}({f_column}) ON DELETE {on_delete_str}"
    ))
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

    let tablename = TableWriter::new(syntax).write(table);
    let colname = ColumnWriter::new(syntax).excape(col.name.as_str());
    let foreign_column = ColumnWriter::new(syntax).excape(foreign_column);

    format!(
        "ALTER TABLE {tablename} ADD CONSTRAINT {indexname} FOREIGN KEY ({colname}) REFERENCES {foreign_table} ({foreign_column}) ON DELETE {on_delete_str}"
    )
}
