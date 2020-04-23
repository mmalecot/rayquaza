//! Error module.

use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

/// Kinds of window creation errors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CreateWindowError {
    InitializationFailed,
    AlreadyCreated,
}

impl Display for CreateWindowError {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "{:?}", self)
    }
}

impl Error for CreateWindowError {}

/// Kinds of texture loading errors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LoadTextureError {
    ReadFailed,
}

impl Display for LoadTextureError {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "{:?}", self)
    }
}

impl Error for LoadTextureError {}
