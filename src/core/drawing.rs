//! Drawing module.

use crate::{
    core::{color::Color, math::Vector2, texture::Texture},
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

    /// Draws a color-filled circle.
    pub fn draw_circle(&mut self, x: i32, y: i32, radius: f32, color: impl Into<Color>) {
        unsafe {
            ffi::DrawCircle(x, y, radius, color.into().into());
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

    /// Draws a color-filled rectangle.
    pub fn draw_rectangle(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        color: impl Into<Color>,
    ) {
        unsafe {
            ffi::DrawRectangle(x, y, width, height, color.into().into());
        }
    }

    /// Draws a Texture.
    pub fn draw_texture(&mut self, texture: &Texture, x: i32, y: i32, color: impl Into<Color>) {
        unsafe { ffi::DrawTexture(texture.raw, x, y, color.into().into()) }
    }
}

impl Drop for Canvas {
    fn drop(&mut self) {
        unsafe {
            ffi::EndDrawing();
        }
    }
}
