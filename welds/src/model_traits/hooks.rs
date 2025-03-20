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
    fn before(&mut self) -> impl std::future::Future<Output = Result<()>> + Send;
}

pub trait BeforeUpdate {
    /// a last minute opportunity to check/edit a model before it is saved to the database
    /// Err results will cancel the action.
    /// you can force a cancel by returning `welds::errors::weldsError::ActionCanceled`
    ///
    /// you can also return any anyhow errors. Useful for things like validation
    fn before(&mut self) -> impl std::future::Future<Output = Result<()>> + Send;
}

pub trait BeforeDelete {
    /// a last minute opportunity to check/edit a model before it is saved to the database
    /// Err results will cancel the action.
    /// you can force a cancel by returning `welds::errors::weldsError::ActionCanceled`
    ///
    /// you can also return any anyhow errors. Useful for things like validation
    fn before(&self) -> impl std::future::Future<Output = Result<()>> + Send;
}

pub trait AfterCreate {
    /// A way go get informed when a model is created in the database.
    ///
    /// is called after a model is created
    fn after(&self) -> impl std::future::Future<Output = Result<()>> + Send;
}

pub trait AfterUpdate {
    /// A way go get informed when a model is updated in the database.
    ///
    /// is called after a model is created
    fn after(&self) -> impl std::future::Future<Output = Result<()>> + Send;
}

pub trait AfterDelete {
    /// A way go get informed when a model is deleted from the database.
    ///
    /// is called after a model is deleted
    fn after(&self) -> impl std::future::Future<Output = Result<()>> + Send;
}
