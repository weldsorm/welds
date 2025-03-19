use crate::errors;
#[cfg(feature = "tracing")]
use tracing;

/// Emits an event when an error is returned from the client, if the "tracing" feature
/// is enabled and a corresponding tracing-subscriber is registered, otherwise no-op
pub fn db_error<T>(result: Result<T, sqlx::Error>) -> Result<T, sqlx::Error> {
    #[cfg(feature = "tracing")]
    if let Err(e) = result {
        tracing::warn!("Database Error; {}", e);
        return Err(e)
    }

    result
}
