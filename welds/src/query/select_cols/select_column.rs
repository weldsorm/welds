use welds_connections::Syntax;
use crate::writers::ColumnWriter;

pub(crate) struct SelectColumn {
    pub(crate) col_name: String,
    pub(crate) field_name: String,
    pub(crate) kind: SelectKind,
}

impl SelectColumn {
    pub fn is_aggregate(&self) -> bool {
        ![SelectKind::All, SelectKind::Column].contains(&self.kind)
    }
}

#[derive(PartialEq)]
pub(crate) enum SelectKind {
    Column,
    All,
    #[cfg(feature = "group-by")]
    Count,
    #[cfg(feature = "group-by")]
    Max,
    #[cfg(feature = "group-by")]
    Min,
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
            #[cfg(feature = "group-by")]
            SelectKind::Count => {
                format!("COUNT({}.{}) AS {}", alias, colname, fieldname)
            }
            #[cfg(feature = "group-by")]
            SelectKind::Max => {
                format!("MAX({}.{}) AS {}", alias, colname, fieldname)
            }
            #[cfg(feature = "group-by")]
            SelectKind::Min => {
                format!("MIN({}.{}) AS {}", alias, colname, fieldname)
            }
        }
    }
}
