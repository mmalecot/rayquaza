//! Shapes module.

use crate::{
    color::Color,
    drawing::Canvas,
    ffi,
    math::{Rectangle, Vector2},
};

/// Shapes.
impl Canvas {
    /// Draws a pixel.
    #[inline]
    pub fn draw_pixel(&mut self, x: i32, y: i32, color: impl Into<Color>) {
        unsafe {
            ffi::DrawPixel(x, y, color.into().into());
        }
    }

    /// Draws a pixel (vector version).
    #[inline]
    pub fn draw_pixel_vec(&mut self, position: impl Into<Vector2>, color: impl Into<Color>) {
        unsafe {
            ffi::DrawPixelV(position.into().into(), color.into().into());
        }
    }

    /// Draws a line.
    #[inline]
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

    /// Draws a line (vector version).
    #[inline]
    pub fn draw_line_vec(
        &mut self,
        start: impl Into<Vector2>,
        end: impl Into<Vector2>,
        color: impl Into<Color>,
    ) {
        unsafe {
            ffi::DrawLineV(start.into().into(), end.into().into(), color.into().into());
        }
    }

    /// Draws a line defining thickness.
    #[inline]
    pub fn draw_line_ex(
        &mut self,
        start: impl Into<Vector2>,
        end: impl Into<Vector2>,
        thick: f32,
        color: impl Into<Color>,
    ) {
        unsafe {
            ffi::DrawLineEx(
                start.into().into(),
                end.into().into(),
                thick,
                color.into().into(),
            );
        }
    }

    /// Draws a line using cubic-bezier curves in-out.
    #[inline]
    pub fn draw_line_bezier(
        &mut self,
        start: impl Into<Vector2>,
        end: impl Into<Vector2>,
        thick: f32,
        color: impl Into<Color>,
    ) {
        unsafe {
            ffi::DrawLineBezier(
                start.into().into(),
                end.into().into(),
                thick,
                color.into().into(),
            );
        }
    }

    /// Draws lines sequence.
    #[inline]
    pub fn draw_line_strip(&mut self, points: &[Vector2], color: impl Into<Color>) {
        unsafe {
            ffi::DrawLineStrip(
                points.as_ptr() as *mut ffi::Vector2,
                points.len() as i32,
                color.into().into(),
            );
        }
    }

    /// Draws a color-filled circle.
    #[inline]
    pub fn draw_circle(
        &mut self,
        center_x: i32,
        center_y: i32,
        radius: f32,
        color: impl Into<Color>,
    ) {
        unsafe {
            ffi::DrawCircle(center_x, center_y, radius, color.into().into());
        }
    }

    /// Draws a piece of a circle.
    #[inline]
    pub fn draw_circle_sector(
        &mut self,
        center: impl Into<Vector2>,
        radius: f32,
        start_angle: i32,
        end_angle: i32,
        segments: i32,
        color: impl Into<Color>,
    ) {
        unsafe {
            ffi::DrawCircleSector(
                center.into().into(),
                radius,
                start_angle,
                end_angle,
                segments,
                color.into().into(),
            );
        }
    }

    /// Draws circle sector outline.
    #[inline]
    pub fn draw_circle_sector_lines(
        &mut self,
        center: impl Into<Vector2>,
        radius: f32,
        start_angle: i32,
        end_angle: i32,
        segments: i32,
        color: impl Into<Color>,
    ) {
        unsafe {
            ffi::DrawCircleSectorLines(
                center.into().into(),
                radius,
                start_angle,
                end_angle,
                segments,
                color.into().into(),
            );
        }
    }

    /// Draws a gradient-filled circle.
    #[inline]
    pub fn draw_circle_gradient(
        &mut self,
        center_x: i32,
        center_y: i32,
        radius: f32,
        color1: impl Into<Color>,
        color2: impl Into<Color>,
    ) {
        unsafe {
            ffi::DrawCircleGradient(
                center_x,
                center_y,
                radius,
                color1.into().into(),
                color2.into().into(),
            );
        }
    }

    /// Draws a color-filled circle (vector version).
    #[inline]
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

    /// Draws circle outline.
    #[inline]
    pub fn draw_circle_lines(
        &mut self,
        center_x: i32,
        center_y: i32,
        radius: f32,
        color: impl Into<Color>,
    ) {
        unsafe {
            ffi::DrawCircleLines(center_x, center_y, radius, color.into().into());
        }
    }

    /// Draws ellipse.
    #[inline]
    pub fn draw_ellipse(
        &mut self,
        center_x: i32,
        center_y: i32,
        horizontal_radius: f32,
        vertical_radius: f32,
        color: impl Into<Color>,
    ) {
        unsafe {
            ffi::DrawEllipse(
                center_x,
                center_y,
                horizontal_radius,
                vertical_radius,
                color.into().into(),
            );
        }
    }

    /// Draws ellipse outline.
    #[inline]
    pub fn draw_ellipse_lines(
        &mut self,
        center_x: i32,
        center_y: i32,
        horizontal_radius: f32,
        vertical_radius: f32,
        color: impl Into<Color>,
    ) {
        unsafe {
            ffi::DrawEllipseLines(
                center_x,
                center_y,
                horizontal_radius,
                vertical_radius,
                color.into().into(),
            );
        }
    }

    /// Draws ring.
    #[inline]
    pub fn draw_ring(
        &mut self,
        center: impl Into<Vector2>,
        inner_radius: f32,
        outer_radius: f32,
        start_angle: i32,
        end_angle: i32,
        segments: i32,
        color: impl Into<Color>,
    ) {
        unsafe {
            ffi::DrawRing(
                center.into().into(),
                inner_radius,
                outer_radius,
                start_angle,
                end_angle,
                segments,
                color.into().into(),
            )
        };
    }

    /// Draws ring out.
    #[inline]
    pub fn draw_ring_lines(
        &mut self,
        center: impl Into<Vector2>,
        inner_radius: f32,
        outer_radius: f32,
        start_angle: i32,
        end_angle: i32,
        segments: i32,
        color: impl Into<Color>,
    ) {
        unsafe {
            ffi::DrawRingLines(
                center.into().into(),
                inner_radius,
                outer_radius,
                start_angle,
                end_angle,
                segments,
                color.into().into(),
            )
        };
    }

    /// Draws a color-filled rectangle.
    #[inline]
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

    /// Draws a color-filled rectangle (vector version).
    #[inline]
    pub fn draw_rectangle_vec(
        &mut self,
        position: impl Into<Vector2>,
        size: impl Into<Vector2>,
        color: impl Into<Color>,
    ) {
        unsafe {
            ffi::DrawRectangleV(
                position.into().into(),
                size.into().into(),
                color.into().into(),
            );
        }
    }

    // Draws a color-filled rectangle.
    #[inline]
    pub fn draw_rectangle_rec(&mut self, rectangle: impl Into<Rectangle>, color: impl Into<Color>) {
        unsafe {
            ffi::DrawRectangleRec(rectangle.into().into(), color.into().into());
        }
    }

    /// Draws a color-filled rectangle with pro parameters.
    #[inline]
    pub fn draw_rectangle_pro(
        &mut self,
        rectangle: impl Into<Rectangle>,
        origin: impl Into<Vector2>,
        rotation: f32,
        color: impl Into<Color>,
    ) {
        unsafe {
            ffi::DrawRectanglePro(
                rectangle.into().into(),
                origin.into().into(),
                rotation,
                color.into().into(),
            );
        }
    }

    /// Draws a vertical-gradient-filled rectangle.
    #[inline]
    pub fn draw_rectangle_vertical_gradient(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        color1: impl Into<Color>,
        color2: impl Into<Color>,
    ) {
        unsafe {
            ffi::DrawRectangleGradientV(
                x,
                y,
                width,
                height,
                color1.into().into(),
                color2.into().into(),
            );
        }
    }

    /// Draws a horizontal-gradient-filled rectangle.
    #[inline]
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

    /// Draws a gradient-filled rectangle with custom vertex colors.
    #[inline]
    pub fn draw_rectangle_gradient_ex(
        &mut self,
        rectangle: impl Into<Rectangle>,
        color1: impl Into<Color>,
        color2: impl Into<Color>,
        color3: impl Into<Color>,
        color4: impl Into<Color>,
    ) {
        unsafe {
            ffi::DrawRectangleGradientEx(
                rectangle.into().into(),
                color1.into().into(),
                color2.into().into(),
                color3.into().into(),
                color4.into().into(),
            );
        }
    }

    /// Draws rectangle outline.
    #[inline]
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

    /// Draws rectangle outline with extended parameters.
    pub fn draw_rectangle_lines_ex(
        &mut self,
        rectangle: impl Into<Rectangle>,
        line_thick: i32,
        color: impl Into<Color>,
    ) {
        unsafe {
            ffi::DrawRectangleLinesEx(rectangle.into().into(), line_thick, color.into().into());
        }
    }

    /// Draws rectangle with rounded edges.
    #[inline]
    pub fn draw_rectangle_rounded(
        &mut self,
        rectangle: impl Into<Rectangle>,
        roundness: f32,
        segments: i32,
        color: impl Into<Color>,
    ) {
        unsafe {
            ffi::DrawRectangleRounded(
                rectangle.into().into(),
                roundness,
                segments,
                color.into().into(),
            );
        }
    }

    /// Draws rectangle with rounded edges outline.
    #[inline]
    pub fn draw_rectangle_rounded_lines(
        &mut self,
        rectangle: impl Into<Rectangle>,
        roundness: f32,
        segments: i32,
        color: impl Into<Color>,
    ) {
        unsafe {
            ffi::DrawRectangleRoundedLines(
                rectangle.into().into(),
                roundness,
                segments,
                color.into().into(),
            );
        }
    }

    /// Draws a color-filled triangle (vertex in counter-clockwise order).
    #[inline]
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
    #[inline]
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

    /// Draws a triangle fan defined by points (first vertex is the center).
    #[inline]
    pub fn draw_triangle_fan(&mut self, points: &[Vector2], color: impl Into<Color>) {
        unsafe {
            ffi::DrawTriangleFan(
                points.as_ptr() as *mut ffi::Vector2,
                points.len() as i32,
                color.into().into(),
            );
        }
    }

    /// Draws a triangle strip defined by points.
    #[inline]
    pub fn draw_triangle_strip(&mut self, points: &[Vector2], color: impl Into<Color>) {
        unsafe {
            ffi::DrawTriangleStrip(
                points.as_ptr() as *mut ffi::Vector2,
                points.len() as i32,
                color.into().into(),
            );
        }
    }

    /// Draws a regular polygon (vector version).
    #[inline]
    pub fn draw_polygon(
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

    /// Draws a polygon outline.
    #[inline]
    pub fn draw_polygon_lines(
        &mut self,
        center: impl Into<Vector2>,
        sides: i32,
        radius: f32,
        rotation: f32,
        color: impl Into<Color>,
    ) {
        unsafe {
            ffi::DrawPolyLines(
                center.into().into(),
                sides,
                radius,
                rotation,
                color.into().into(),
            );
        }
    }
}
