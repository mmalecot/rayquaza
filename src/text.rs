//! Text utilities.

use crate::{color::Color, drawing::Canvas, ffi};
use std::ffi::CString;

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
}

/// Measures text width for default font.
pub fn measure_text(text: &str, size: i32) -> i32 {
    unsafe {
        let text = CString::new(text).unwrap();
        ffi::MeasureText(text.as_ptr(), size)
    }
}
