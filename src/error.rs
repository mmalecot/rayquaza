//! Error module.

use crate::{texture::LoadTextureError, window::CreateWindowError};
use std::fmt::{Display, Formatter, Result};

/// Generic error type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    CreateWindowError(CreateWindowError),
    LoadTextureError(LoadTextureError),
}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "{:?}", self)
    }
}

impl std::error::Error for Error {}

impl From<CreateWindowError> for Error {
    fn from(error: CreateWindowError) -> Error {
        Error::CreateWindowError(error)
    }
}

impl From<LoadTextureError> for Error {
    fn from(error: LoadTextureError) -> Error {
        Error::LoadTextureError(error)
    }
}
