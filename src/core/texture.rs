//! Texture module.

use crate::{core::window::Handle, ffi};
use std::rc::Rc;

/// 2D Texture.
pub struct Texture {
    pub(crate) _handle: Rc<Handle>,
    pub(crate) raw: ffi::Texture2D,
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { ffi::UnloadTexture(self.raw) }
    }
}
