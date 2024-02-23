use crate::detect::TableDef;

mod sqlite_writer;

pub struct Drop {
    pub(super) tabledef: TableDef,
}

impl Drop {
    pub(crate) fn new(tabledef: TableDef) -> Drop {
        Drop { tabledef }
    }
}
