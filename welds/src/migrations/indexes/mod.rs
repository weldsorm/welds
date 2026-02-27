// builder pattern style create index
use crate::migrations::MigrationWriter;
use crate::model_traits::TableIdent;
use crate::writers::{ColumnWriter, TableWriter};

#[derive(Clone)]
pub struct CreateIndex {
    name: String,
}

pub fn create_index() -> CreateIndex {
    CreateIndex { name: "".into() }
}

impl CreateIndex {
    /// The table to create the index on
    pub fn table(self, table: impl Into<String>) -> CreateIndexWithTable {
        CreateIndexWithTable {
            name: self.name,
            table: table.into(),
            unique: false,
        }
    }
}

#[derive(Clone)]
pub struct CreateIndexWithTable {
    name: String,
    table: String,
    unique: bool,
}

impl CreateIndexWithTable {
    /// Creates a unique index. Insures uniqueness over columns in index
    pub fn unique(mut self) -> CreateIndexWithTable {
        self.unique = true;
        self
    }

    /// The column to add the index for
    pub fn column(self, column: impl Into<String>) -> CreateIndexWithTableColumn {
        CreateIndexWithTableColumn {
            name: self.name,
            table: self.table,
            columns: vec![column.into()],
            includes: Vec::default(),
            unique: false,
        }
    }
}

#[derive(Clone)]
pub struct CreateIndexWithTableColumn {
    name: String,
    table: String,
    columns: Vec<String>,
    includes: Vec<String>,
    unique: bool,
}

impl CreateIndexWithTableColumn {
    /// Add another column to this index
    pub fn column(self, column: impl Into<String>) -> CreateIndexWithTableColumn {
        let mut columns = self.columns;
        columns.push(column.into());
        CreateIndexWithTableColumn {
            name: self.name,
            table: self.table,
            columns,
            includes: self.includes,
            unique: self.unique,
        }
    }

    /// Include the contents of another column in the results of this index.
    pub fn include(self, column: impl Into<String>) -> CreateIndexWithTableColumn {
        let mut includes = self.includes;
        includes.push(column.into());
        CreateIndexWithTableColumn {
            name: self.name,
            table: self.table,
            columns: self.columns,
            includes,
            unique: self.unique,
        }
    }

    /// The name to use for this index.
    fn index_name(&self) -> String {
        if !self.name.is_empty() {
            return self.name.to_owned();
        }
        let table = to_snake_case(&self.table);
        let cols: Vec<String> = self.columns.iter().map(|x| to_snake_case(x)).collect();
        let cols = cols.join("_");
        let name = format!("idx_{table}_{cols}");
        truncate_utf8(&name, 60)
    }
}

impl MigrationWriter for CreateIndexWithTableColumn {
    fn up_sql(&self, syntax: welds_connections::Syntax) -> Vec<String> {
        let ident = TableIdent::parse(&self.table);
        let tablename: String = TableWriter::new(syntax).write(&ident);
        let cw = ColumnWriter::new(syntax);
        let cols: Vec<String> = self.columns.iter().map(|c| cw.excape(c)).collect();
        let cols: String = cols.join(",");
        let includes: Vec<String> = self.includes.iter().map(|c| cw.excape(c)).collect();
        let includes: String = includes.join(",");
        let name = self.index_name();

        let mut parts: Vec<&str> = Vec::default();
        parts.push("CREATE");
        if self.unique {
            parts.push("UNIQUE");
        }
        parts.push("INDEX");
        parts.push(&name);
        parts.push("ON");
        parts.push(&tablename);
        parts.push("(");
        parts.push(&cols);
        parts.push(")");

        vec![parts.join(" ")]
    }

    fn down_sql(&self, _syntax: welds_connections::Syntax) -> Vec<String> {
        let name = self.index_name();
        let sql = format!("DROP INDEX {name}");
        vec![sql]
    }
}

// quick and dirty snake case for string
fn to_snake_case(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut prev_was_lower = false;

    for c in input.chars() {
        if c.is_ascii_alphanumeric() {
            if c.is_ascii_uppercase() {
                if prev_was_lower {
                    out.push('_');
                }
                out.push(c.to_ascii_lowercase());
                prev_was_lower = false;
            } else {
                out.push(c);
                prev_was_lower = true;
            }
        } else {
            if !out.ends_with('_') {
                out.push('_');
            }
            prev_was_lower = false;
        }
    }

    out.trim_matches('_').to_string()
}

/// truncate a string so it doesn't exceed the max length for names
fn truncate_utf8(s: &str, max_bytes: usize) -> String {
    if s.len() <= max_bytes {
        return s.to_owned();
    }
    let mut end = max_bytes;
    // Walk backwards until we hit a valid UTF-8 boundary
    while !s.is_char_boundary(end) {
        end -= 1;
    }
    s[..end].to_string()
}

#[cfg(test)]
mod tests;
