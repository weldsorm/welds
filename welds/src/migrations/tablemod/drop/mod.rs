use crate::Syntax;
use crate::detect::TableDef;
use crate::migrations::MigrationWriter;
use crate::migrations::writers;

impl MigrationWriter for Drop {
    fn down_sql(&self, syntax: Syntax) -> Vec<String> {
        let tabledef = match &self.tabledef {
            Some(x) => x,
            None => return vec![],
        };
        writers::create_table::from_def(syntax, tabledef)
    }

    fn up_sql(&self, syntax: Syntax) -> Vec<String> {
        let tabledef = match &self.tabledef {
            Some(x) => x,
            None => return vec![],
        };
        let tablename = tabledef.ident();
        vec![writers::drop_table(syntax, tablename)]
    }
}

mod writer;

pub struct Drop {
    pub(super) tabledef: Option<TableDef>,
}

impl Drop {
    pub(crate) fn new(tabledef: Option<TableDef>) -> Drop {
        Drop { tabledef }
    }
}
