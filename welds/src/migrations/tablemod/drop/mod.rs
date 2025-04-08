use crate::Syntax;
use crate::detect::TableDef;
use crate::migrations::MigrationWriter;
use crate::migrations::writers;

impl MigrationWriter for Drop {
    fn down_sql(&self, syntax: Syntax) -> Vec<String> {
        writers::create_table::from_def(syntax, &self.tabledef)
    }

    fn up_sql(&self, _syntax: Syntax) -> Vec<String> {
        let tablename = self.tabledef.ident();
        vec![writers::drop_table(tablename)]
    }
}

mod writer;

pub struct Drop {
    pub(super) tabledef: TableDef,
}

impl Drop {
    pub(crate) fn new(tabledef: TableDef) -> Drop {
        Drop { tabledef }
    }
}
