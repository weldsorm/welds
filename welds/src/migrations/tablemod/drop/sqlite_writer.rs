use super::*;
use crate::migrations::MigrationWriter;

use super::super::change::sqlite_writer::{build_drop, build_table_create, old_columns};

impl MigrationWriter<sqlx::Sqlite> for Drop {
    fn down_sql(&self) -> Vec<String> {
        let old_cols = old_columns(&self.tabledef);
        let tablename = self.tabledef.ident().to_string();
        vec![build_table_create(&tablename, &old_cols)]
    }

    fn up_sql(&self) -> Vec<String> {
        let tablename = self.tabledef.ident().to_string();
        vec![build_drop(&tablename)]
    }
}
