use crate::writers::ColumnWriter;
use welds_connections::Syntax;

/// What is added to the query_builder, will be build into SQL
pub(crate) struct SelectColumn {
    pub(crate) col_name: String,
    pub(crate) field_name: String,
    pub(crate) kind: SelectKind,
}

impl SelectColumn {
    pub fn is_aggregate(&self) -> bool {
        self.kind.is_aggregate()
    }
}
impl SelectRender {
    pub fn is_aggregate(&self) -> bool {
        self.kind.is_aggregate()
    }
}

impl SelectKind {
    pub fn is_aggregate(&self) -> bool {
        ![SelectKind::All, SelectKind::Column].contains(&self)
    }
}

#[derive(PartialEq, Clone, Debug)]
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

/// used while writing SQL to help keep track of parts of the select
/// it is the parts needed to know how to write each part of the select
#[derive(Debug)]
pub(crate) struct SelectRender {
    pub(crate) col_name: String,
    pub(crate) field_name: String,
    pub(crate) alias: String,
    pub(crate) kind: SelectKind,
}

impl SelectRender {
    pub(crate) fn new(col: &SelectColumn, alias: impl Into<String>) -> Self {
        Self {
            col_name: col.col_name.to_owned(),
            field_name: col.field_name.to_owned(),
            alias: alias.into(),
            kind: col.kind.clone(),
        }
    }

    pub(crate) fn write(&self, syntax: Syntax) -> String {
        let writer = ColumnWriter::new(syntax);
        let colname = writer.excape(&self.col_name);
        let fieldname = writer.excape(&self.field_name);

        match self.kind {
            SelectKind::Column => {
                if colname == fieldname {
                    format!("{}.{}", self.alias, colname)
                } else {
                    format!("{}.{} AS {}", self.alias, colname, fieldname)
                }
            }
            SelectKind::All => {
                format!("{}.*", self.alias)
            }
            #[cfg(feature = "unstable-api")]
            SelectKind::Count => {
                format!("COUNT({}.{}) AS {}", self.alias, colname, fieldname)
            }
            #[cfg(feature = "unstable-api")]
            SelectKind::Max => {
                format!("MAX({}.{}) AS {}", self.alias, colname, fieldname)
            }
            #[cfg(feature = "unstable-api")]
            SelectKind::Min => {
                format!("MIN({}.{}) AS {}", self.alias, colname, fieldname)
            }
        }
    }
}
