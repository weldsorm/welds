use crate::errors::Result;

/// A collection of trait that allow for intercepting/monitoring call to the database
///
/// These are AUTOMATICALLY implemented by the Welds Macros
/// ALL models get these traits.
///
/// The implementation for these derived traits differed based on if you have told
/// Welds you want a Hook in the welds macros
///
/// WARNING: These are NOT effected by bulk operations !!!
pub trait BeforeCreate {
    /// a last minute opportunity to check/edit a model before it is saved to the database
    /// Err results will cancel the action.
    /// you can force a cancel by returning `welds::errors::weldsError::ActionCanceled`
    ///
    /// you can also return any anyhow errors. Useful for things like validation
    #[cfg(not(feature = "__sync"))]
    fn before(&mut self) -> impl std::future::Future<Output = Result<()>> + Send;
    #[maybe_async::sync_impl]
    fn before(&mut self) -> Result<()>;
}

#[maybe_async::maybe_async]
pub trait BeforeUpdate {
    /// a last minute opportunity to check/edit a model before it is saved to the database
    /// Err results will cancel the action.
    /// you can force a cancel by returning `welds::errors::weldsError::ActionCanceled`
    ///
    /// you can also return any anyhow errors. Useful for things like validation
    #[cfg(not(feature = "__sync"))]
    fn before(&mut self) -> impl std::future::Future<Output = Result<()>> + Send;
    #[maybe_async::sync_impl]
    fn before(&mut self) -> Result<()>;
}

pub trait BeforeDelete {
    /// a last minute opportunity to check/edit a model before it is saved to the database
    /// Err results will cancel the action.
    /// you can force a cancel by returning `welds::errors::weldsError::ActionCanceled`
    ///
    /// you can also return any anyhow errors. Useful for things like validation
    #[cfg(not(feature = "__sync"))]
    fn before(&self) -> impl std::future::Future<Output = Result<()>> + Send;
    #[maybe_async::sync_impl]
    fn before(&self) -> Result<()>;
}

pub trait AfterCreate {
    /// A way go get informed when a model is created in the database.
    ///
    /// is called after a model is created
    #[cfg(not(feature = "__sync"))]
    fn after(&self) -> impl std::future::Future<Output = Result<()>> + Send;
    #[maybe_async::sync_impl]
    fn after(&self) -> Result<()>;
}

pub trait AfterUpdate {
    /// A way go get informed when a model is updated in the database.
    ///
    /// is called after a model is created
    #[cfg(not(feature = "__sync"))]
    fn after(&self) -> impl std::future::Future<Output = Result<()>> + Send;
    #[maybe_async::sync_impl]
    fn after(&self) -> Result<()>;
}

pub trait AfterDelete {
    /// A way go get informed when a model is deleted from the database.
    ///
    /// is called after a model is deleted
    #[cfg(not(feature = "__sync"))]
    fn after(&self) -> impl std::future::Future<Output = Result<()>> + Send;
    #[maybe_async::sync_impl]
    fn after(&self) -> Result<()>;
}
