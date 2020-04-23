//! Result module.

/// Convenient result type consisting of a return type and an error.
pub type Result<T = (), E = crate::error::Error> = std::result::Result<T, E>;
