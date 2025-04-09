mod bulk;
mod single;

pub use single::insert_one;

pub use bulk::run_with_ids as bulk_insert_with_ids;
pub use bulk::run_without_ids as bulk_insert;
