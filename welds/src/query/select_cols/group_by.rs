#[derive(Clone)]
pub(crate) struct GroupBy {
    pub(crate) col_name: String,
    pub(crate) table_alias: Option<String>,
}

impl GroupBy {
    pub(crate) fn new(col: &str) -> Self {
        Self {
            col_name: col.to_string(),
            table_alias: None,
        }
    }

    pub(crate) fn set_alias(mut self, alias: &str) -> Self {
        self.table_alias = Some(alias.into());
        self
    }
}
