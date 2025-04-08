use crate::writers::ColumnWriter;
use welds_connections::Syntax;

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
    #[cfg(feature = "unstable-api")]
    Count,
    #[cfg(feature = "unstable-api")]
    Max,
    #[cfg(feature = "unstable-api")]
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
            #[cfg(feature = "unstable-api")]
            SelectKind::Count => {
                format!("COUNT({}.{}) AS {}", alias, colname, fieldname)
            }
            #[cfg(feature = "unstable-api")]
            SelectKind::Max => {
                format!("MAX({}.{}) AS {}", alias, colname, fieldname)
            }
            #[cfg(feature = "unstable-api")]
            SelectKind::Min => {
                format!("MIN({}.{}) AS {}", alias, colname, fieldname)
            }
        }
    }
}
