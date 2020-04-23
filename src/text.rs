//! Text module.

use crate::{color::Color, drawing::Canvas, ffi};
use std::ffi::CString;

/// Text.
impl Canvas {
    /// Draws text using default font.
    pub fn draw_text(&mut self, text: &str, x: i32, y: i32, size: i32, color: impl Into<Color>) {
        unsafe {
            let text = CString::new(text).unwrap();
            ffi::DrawText(text.as_ptr(), x, y, size, color.into().into());
        }
    }
}
