use welds_connections::Syntax;
use crate::writers::ColumnWriter;

pub(crate) struct SelectColumn {
    pub(crate) col_name: String,
    pub(crate) field_name: String,
    pub(crate) kind: SelectKind,
}

pub(crate) enum SelectKind {
    Column,
    Count,
    Max,
    Min,
    All,
}

impl SelectColumn {
    pub(crate) fn write(&self, syntax: Syntax, alias: &str) -> String {
        let writer = ColumnWriter::new(syntax);
        let colname = writer.excape(&self.col_name);
        let fieldname = writer.excape(&self.field_name);

        match self.kind {
            SelectKind::Column => {
                if colname == fieldname {
                    format!("{}.{}", alias, colname)
                } else {
                    format!("{}.{} AS {}", alias, colname, fieldname)
                }
            }
            SelectKind::All => {
                format!("{}.*", alias)
            }
            SelectKind::Count => {
                format!("COUNT({}.{}) AS {}", alias, colname, fieldname)
            }
            SelectKind::Max => {
                format!("MAX({}.{}) AS {}", alias, colname, fieldname)
            }
            SelectKind::Min => {
                format!("MIN({}.{}) AS {}", alias, colname, fieldname)
            }
        }
    }
}
