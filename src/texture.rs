//! Texture module.

use crate::{
    color::Color,
    drawing::Canvas,
    ffi,
    window::{Handle, Window},
    LoadTextureError,
};
use std::{ffi::CString, path::Path, rc::Rc};

/// 2D texture.
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

/// Texture.
impl Canvas {
    /// Draws a texture.
    #[inline]
    pub fn draw_texture(&mut self, texture: &Texture, x: i32, y: i32, color: impl Into<Color>) {
        unsafe { ffi::DrawTexture(texture.raw, x, y, color.into().into()) }
    }
}

/// Texture.
impl Window {
    /// Loads texture from file into GPU memory (VRAM).
    pub fn load_texture<P: AsRef<Path>>(&self, path: P) -> Result<Texture, LoadTextureError> {
        unsafe {
            let path = CString::new(path.as_ref().display().to_string()).unwrap();
            let raw = ffi::LoadTexture(path.as_ptr());
            if raw.id != 0 {
                Ok(Texture {
                    _handle: self.handle.clone(),
                    raw,
                })
            } else {
                Err(LoadTextureError::ReadFailed)
            }
        }
    }
}
