mod bulk;
mod single;

pub use single::insert_one;

pub use bulk::bulk_insert;
pub use bulk::bulk_insert_with_ids;

#[cfg(feature = "unstable-api")]
pub use bulk::bulk_insert_override_tablename_unsafe;
#[cfg(feature = "unstable-api")]
pub use bulk::bulk_insert_with_ids_override_tablename_unsafe;
