use super::column::ColumnWriter;
use crate::Syntax;
use crate::model_traits::TableIdent;

pub struct TableWriter {
    syntax: Syntax,
}

impl TableWriter {
    pub fn new(syntax: Syntax) -> Self {
        Self { syntax }
    }

    /// Returns the String that should be used when executing SQL
    pub fn write(&self, ident: &TableIdent) -> String {
        let excaper = ColumnWriter::new(self.syntax);

        // some people like pain
        let table_name = if ident.name().contains(" ") {
            excaper.excape(ident.name())
        } else {
            ident.name().to_owned()
        };

        let schema_name = ident.schema().map(|s| {
            if s.contains(" ") {
                excaper.excape(s)
            } else {
                s.to_owned()
            }
        });

        match schema_name {
            Some(schema_name) => format!("{schema_name}.{table_name}"),
            None => table_name,
        }
    }

    /// Returns the String that should be used when executing SQL
    pub fn write2(&self, parts: &[&str]) -> String {
        let excaper = ColumnWriter::new(self.syntax);

        let cleaned_parts: Vec<String> = parts
            .iter()
            .copied()
            .map(|p| {
                if p.contains(" ") {
                    excaper.excape(p)
                } else {
                    p.to_owned()
                }
            })
            .collect();

        cleaned_parts.join(".")
    }
}
