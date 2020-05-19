//! Text utilities.

use crate::{
    color::Color,
    drawing::Canvas,
    ffi,
    math::{Rectangle, Vector2},
    window::{Handle, Window},
};
use std::{ffi::CString, fmt, path::Path, ptr, rc::Rc};

/// Kinds of font loading errors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LoadFontError {
    ReadFailed,
}

impl fmt::Display for LoadFontError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{:?}", self)
    }
}

impl std::error::Error for LoadFontError {}

/// Font.
pub struct Font {
    pub(crate) _handle: Rc<Handle>,
    pub(crate) raw: ffi::Font,
}

impl Drop for Font {
    fn drop(&mut self) {
        unsafe {
            ffi::UnloadFont(self.raw);
        }
    }
}

/// Text.
impl Canvas {
    /// Shows current FPS.
    #[inline]
    pub fn draw_fps(&mut self, x: i32, y: i32) {
        unsafe {
            ffi::DrawFPS(x, y);
        }
    }

    /// Draws text using default font.
    pub fn draw_text(&mut self, text: &str, x: i32, y: i32, size: i32, color: impl Into<Color>) {
        unsafe {
            let text = CString::new(text).unwrap();
            ffi::DrawText(text.as_ptr(), x, y, size, color.into().into());
        }
    }

    /// Draws text with additional parameters.
    pub fn draw_text_ex(
        &mut self,
        font: &Font,
        text: &str,
        position: impl Into<Vector2>,
        size: f32,
        spacing: f32,
        color: impl Into<Color>,
    ) {
        unsafe {
            let text = CString::new(text).unwrap();
            ffi::DrawTextEx(
                font.raw,
                text.as_ptr(),
                position.into().into(),
                size,
                spacing,
                color.into().into(),
            );
        }
    }

    /// Draws text inside rectangle limits.
    #[allow(clippy::too_many_arguments)]
    pub fn draw_text_rec(
        &mut self,
        font: &Font,
        text: &str,
        rectangle: impl Into<Rectangle>,
        size: f32,
        spacing: f32,
        word_wrap: bool,
        color: impl Into<Color>,
    ) {
        unsafe {
            let text = CString::new(text).unwrap();
            ffi::DrawTextRec(
                font.raw,
                text.as_ptr(),
                rectangle.into().into(),
                size,
                spacing,
                word_wrap,
                color.into().into(),
            );
        }
    }
}

/// Text.
impl Window {
    /// Loads font from file into GPU memory (VRAM).
    pub fn load_font<P: AsRef<Path>>(&self, path: P) -> Result<Font, LoadFontError> {
        unsafe {
            let path = CString::new(path.as_ref().display().to_string()).unwrap();
            let raw = ffi::LoadFont(path.as_ptr());
            if raw.texture.id != 0 {
                Ok(Font {
                    _handle: self.handle.clone(),
                    raw,
                })
            } else {
                Err(LoadFontError::ReadFailed)
            }
        }
    }

    /// Loads font from file into GPU memory (VRAM) with extended parameters.
    pub fn load_font_ex<P: AsRef<Path>>(&self, path: P, size: i32) -> Result<Font, LoadFontError> {
        unsafe {
            let path = CString::new(path.as_ref().display().to_string()).unwrap();
            let raw = ffi::LoadFontEx(path.as_ptr(), size, ptr::null_mut(), 0);
            if raw.texture.id != 0 {
                Ok(Font {
                    _handle: self.handle.clone(),
                    raw,
                })
            } else {
                Err(LoadFontError::ReadFailed)
            }
        }
    }

    /// Measures text width for default font.
    pub fn measure_text(&self, text: &str, size: i32) -> i32 {
        unsafe {
            let text = CString::new(text).unwrap();
            ffi::MeasureText(text.as_ptr(), size)
        }
    }

    /// Measures text width with extended parameters.
    pub fn measure_text_ex(&self, font: &Font, text: &str, size: f32, spacing: f32) -> Vector2 {
        unsafe {
            let text = CString::new(text).unwrap();
            ffi::MeasureTextEx(font.raw, text.as_ptr(), size, spacing).into()
        }
    }
}
