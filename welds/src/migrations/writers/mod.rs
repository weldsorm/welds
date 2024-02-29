use crate::model_traits::TableIdent;

pub mod create_table;

mod create_pk_index;
pub use create_pk_index::write as create_pk_index;

pub fn drop_table(table: &TableIdent) -> String {
    format!("DROP TABLE {}", table)
}
