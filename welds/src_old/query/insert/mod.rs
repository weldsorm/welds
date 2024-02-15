mod bulk;
mod single;

pub use single::insert_one;

pub use bulk::run as bulk_insert;
pub use bulk::run_fast as bulk_insert_fast;
