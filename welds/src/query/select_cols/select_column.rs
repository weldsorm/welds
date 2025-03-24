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
