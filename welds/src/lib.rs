pub mod errors;
pub use errors::WeldsError;
pub mod model_traits;
pub mod query;
pub mod relations;
pub mod state;
pub mod writers;

#[cfg(feature = "detect")]
pub mod detect;

#[cfg(feature = "check")]
pub mod check;

#[cfg(feature = "migrations")]
pub mod migrations;

pub use welds_connections as connections;
/// Re-export welds_connections
pub use welds_connections::{Client, Row, Syntax, TransactStart};

/// Re-export the Macro used to make models
pub use welds_macros::WeldsModel;
