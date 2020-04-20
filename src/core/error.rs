//! Error module.

/// Kinds of errors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    WindowInitializationFailed,
    WindowAlreadyCreated,
}
