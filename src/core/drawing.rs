//! Drawing module.

use crate::{
    core::{color::Color, math::Vector2},
    ffi,
};
use std::ffi::CString;

pub struct Canvas;

impl Canvas {
    /// Creates an new canvas to start drawing.
    pub(crate) fn new() -> Canvas {
        unsafe {
            ffi::BeginDrawing();
            Canvas
        }
    }

    /// Sets background color.
    pub fn clear_background(&mut self, color: impl Into<Color>) {
        unsafe { ffi::ClearBackground(color.into().into()) }
    }

    /// Draws text using default font.
    pub fn draw_text(&mut self, text: &str, x: i32, y: i32, size: i32, color: impl Into<Color>) {
        unsafe {
            let text = CString::new(text).unwrap();
            ffi::DrawText(text.as_ptr(), x, y, size, color.into().into());
        }
    }

    /// Draws a color-filled circle (vector version).
    pub fn draw_circle_vec(
        &mut self,
        center: impl Into<Vector2>,
        radius: f32,
        color: impl Into<Color>,
    ) {
        unsafe {
            ffi::DrawCircleV(center.into().into(), radius, color.into().into());
        }
    }
}

impl Drop for Canvas {
    fn drop(&mut self) {
        unsafe {
            ffi::EndDrawing();
        }
    }
}
