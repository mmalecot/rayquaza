//! Camera utilities.

use crate::{ffi, math::Vector2};
use std::mem;

/// 2D camera.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Camera2D {
    pub offset: Vector2,
    pub target: Vector2,
    pub rotation: f32,
    pub zoom: f32,
}

impl Camera2D {
    /// Creates a `Camera2D`.
    #[inline]
    pub const fn new(offset: Vector2, target: Vector2, rotation: f32, zoom: f32) -> Camera2D {
        Camera2D {
            offset,
            target,
            rotation,
            zoom,
        }
    }
}

impl From<&Camera2D> for Camera2D {
    #[inline]
    fn from(camera: &Camera2D) -> Camera2D {
        *camera
    }
}

impl From<ffi::Camera2D> for Camera2D {
    #[inline]
    fn from(camera: ffi::Camera2D) -> Camera2D {
        unsafe { mem::transmute(camera) }
    }
}

impl Into<ffi::Camera2D> for Camera2D {
    #[inline]
    fn into(self) -> ffi::Camera2D {
        unsafe { mem::transmute(self) }
    }
}
