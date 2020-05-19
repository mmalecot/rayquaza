//! Collision detection.

use crate::{
    ffi,
    math::{Rectangle, Vector2},
};

/// Checks collision between two rectangles.
#[inline]
pub fn check_rectangles(
    rectangle1: impl Into<Rectangle>,
    rectangle2: impl Into<Rectangle>,
) -> bool {
    unsafe { ffi::CheckCollisionRecs(rectangle1.into().into(), rectangle2.into().into()) }
}

/// Checks collision between two circles.
#[inline]
pub fn check_circles(
    center1: impl Into<Vector2>,
    radius1: f32,
    center2: impl Into<Vector2>,
    radius2: f32,
) -> bool {
    unsafe {
        ffi::CheckCollisionCircles(
            center1.into().into(),
            radius1,
            center2.into().into(),
            radius2,
        )
    }
}

/// Checks collision between a circle and a rectangle.
#[inline]
pub fn check_circle_rectangle(
    center: impl Into<Vector2>,
    radius: f32,
    rectangle: impl Into<Rectangle>,
) -> bool {
    unsafe { ffi::CheckCollisionCircleRec(center.into().into(), radius, rectangle.into().into()) }
}

/// Gets collision rectangle for two rectangles collision.
#[inline]
pub fn collision_rectangle(
    rectangle1: impl Into<Rectangle>,
    rectangle2: impl Into<Rectangle>,
) -> Rectangle {
    unsafe { ffi::GetCollisionRec(rectangle1.into().into(), rectangle2.into().into()).into() }
}

/// Checks if a point is inside a rectangle.
#[inline]
pub fn check_point_rectangle(point: impl Into<Vector2>, rectangle: impl Into<Rectangle>) -> bool {
    unsafe { ffi::CheckCollisionPointRec(point.into().into(), rectangle.into().into()) }
}

/// Checks if a point is inside a circle.
#[inline]
pub fn check_point_circle(
    point: impl Into<Vector2>,
    center: impl Into<Vector2>,
    radius: f32,
) -> bool {
    unsafe { ffi::CheckCollisionPointCircle(point.into().into(), center.into().into(), radius) }
}

/// Checks if a point is inside a triangle.
#[inline]
pub fn check_point_triangle(
    point: impl Into<Vector2>,
    point1: impl Into<Vector2>,
    point2: impl Into<Vector2>,
    point3: impl Into<Vector2>,
) -> bool {
    unsafe {
        ffi::CheckCollisionPointTriangle(
            point.into().into(),
            point1.into().into(),
            point2.into().into(),
            point3.into().into(),
        )
    }
}
