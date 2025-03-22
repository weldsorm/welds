use crate::errors;
#[cfg(feature = "tracing")]
use tracing;

/// Emits an event when an error is returned from the client, if the "tracing" feature
/// is enabled and a corresponding tracing-subscriber is registered, otherwise no-op
pub(crate) fn db_error<T, E>(result: Result<T, E>) -> Result<T, E>
where
    E: std::fmt::Display,
{
    #[cfg(feature = "tracing")]
    if let Err(e) = result {
        tracing::warn!("Database Error; {}", e);
        return Err(e);
    }
    result
}
