//! Shapes module.

use crate::{color::Color, drawing::Canvas, ffi, math::Vector2};

/// Shapes.
impl Canvas {
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
}
