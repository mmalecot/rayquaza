use crate::{core::color::Color, ffi};
use std::ffi::CString;

pub struct Drawing;

impl Drawing {
    pub(crate) fn new() -> Drawing {
        unsafe {
            ffi::BeginDrawing();
            Drawing
        }
    }

    /// Set background color (framebuffer clear color).
    pub fn clear_background(&self, color: Color) {
        unsafe { ffi::ClearBackground(color.into()) }
    }

    /// Draw text (using default font).
    pub fn draw_text(&self, text: &str, x: i32, y: i32, size: i32, color: Color) {
        unsafe {
            let text = CString::new(text).unwrap();
            ffi::DrawText(text.as_ptr(), x, y, size, color.into());
        }
    }
}

impl Drop for Drawing {
    fn drop(&mut self) {
        unsafe {
            ffi::EndDrawing();
        }
    }
}
