//! Result module.

/// Convenient result type consisting of a return type and an `Error`.
pub type Result<T = ()> = std::result::Result<T, crate::error::Error>;
