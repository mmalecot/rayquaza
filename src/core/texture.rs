//! Texture module.

use crate::{core::window::Handle, ffi};
use std::rc::Rc;

/// 2D Texture.
pub struct Texture {
    pub(crate) _handle: Rc<Handle>,
    pub(crate) raw: ffi::Texture2D,
}

impl Texture {
    /// Returns the texture width.
    pub fn get_width(&self) -> i32 {
        self.raw.width
    }

    /// Returns the texture height.
    pub fn get_height(&self) -> i32 {
        self.raw.height
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { ffi::UnloadTexture(self.raw) }
    }
}
