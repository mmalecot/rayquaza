//! Shapes module.

use crate::{color::Color, drawing::Canvas, ffi, math::Vector2};

/// Shapes.
impl Canvas {
    /// Draws a pixel.
    pub fn draw_pixel(&mut self, x: i32, y: i32, color: impl Into<Color>) {
        unsafe {
            ffi::DrawPixel(x, y, color.into().into());
        }
    }

    /// Draws a pixel (vector version).
    pub fn draw_pixel_vec(&mut self, vector: impl Into<Vector2>, color: impl Into<Color>) {
        unsafe {
            ffi::DrawPixelV(vector.into().into(), color.into().into());
        }
    }

    /// Draws a color-filled circle.
    pub fn draw_line(
        &mut self,
        start_x: i32,
        start_y: i32,
        end_x: i32,
        end_y: i32,
        color: impl Into<Color>,
    ) {
        unsafe {
            ffi::DrawLine(start_x, start_y, end_x, end_y, color.into().into());
        }
    }

    /// Draws a color-filled circle.
    pub fn draw_circle(&mut self, x: i32, y: i32, radius: f32, color: impl Into<Color>) {
        unsafe {
            ffi::DrawCircle(x, y, radius, color.into().into());
        }
    }

    /// Draws a gradient-filled circle.
    pub fn draw_circle_gradient(
        &mut self,
        x: i32,
        y: i32,
        radius: f32,
        color1: impl Into<Color>,
        color2: impl Into<Color>,
    ) {
        unsafe {
            ffi::DrawCircleGradient(x, y, radius, color1.into().into(), color2.into().into());
        }
    }

    /// Draws circle outline.
    pub fn draw_circle_lines(&mut self, x: i32, y: i32, radius: f32, color: impl Into<Color>) {
        unsafe {
            ffi::DrawCircleLines(x, y, radius, color.into().into());
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

    /// Draws rectangle outline.
    pub fn draw_rectangle_lines(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        color: impl Into<Color>,
    ) {
        unsafe {
            ffi::DrawRectangleLines(x, y, width, height, color.into().into());
        }
    }

    /// Draws a horizontal-gradient-filled rectangle.
    pub fn draw_rectangle_horizontal_gradient(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        color1: impl Into<Color>,
        color2: impl Into<Color>,
    ) {
        unsafe {
            ffi::DrawRectangleGradientH(
                x,
                y,
                width,
                height,
                color1.into().into(),
                color2.into().into(),
            );
        }
    }

    /// Draws a color-filled triangle (vertex in counter-clockwise order).
    pub fn draw_triangle(
        &mut self,
        vector1: impl Into<Vector2>,
        vector2: impl Into<Vector2>,
        vector3: impl Into<Vector2>,
        color: impl Into<Color>,
    ) {
        unsafe {
            ffi::DrawTriangle(
                vector1.into().into(),
                vector2.into().into(),
                vector3.into().into(),
                color.into().into(),
            );
        }
    }

    /// Draws triangle outline (vertex in counter-clockwise order).
    pub fn draw_triangle_lines(
        &mut self,
        vector1: impl Into<Vector2>,
        vector2: impl Into<Vector2>,
        vector3: impl Into<Vector2>,
        color: impl Into<Color>,
    ) {
        unsafe {
            ffi::DrawTriangleLines(
                vector1.into().into(),
                vector2.into().into(),
                vector3.into().into(),
                color.into().into(),
            );
        }
    }

    /// Draws a regular polygon (vector version).
    pub fn draw_polygon_vec(
        &mut self,
        center: impl Into<Vector2>,
        sides: i32,
        radius: f32,
        rotation: f32,
        color: impl Into<Color>,
    ) {
        unsafe {
            ffi::DrawPoly(
                center.into().into(),
                sides,
                radius,
                rotation,
                color.into().into(),
            );
        }
    }
}
