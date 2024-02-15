use crate::query::clause::DbParam;
use crate::writers::DbColumnWriter;
use crate::writers::DbCountWriter;
use crate::writers::DbInsertWriter;
use crate::writers::DbLimitSkipWriter;

/// Useful trait to help with making generic DB connections.
/// Use this instead of sqlx::Database so that all other requirements
/// welds functions are meet
pub trait Database:
    sqlx::Database + DbParam + DbColumnWriter + DbLimitSkipWriter + DbCountWriter + DbInsertWriter
{
}

impl<T> Database for T where
    T: sqlx::Database
        + DbParam
        + DbColumnWriter
        + DbLimitSkipWriter
        + DbCountWriter
        + DbInsertWriter
{
}
