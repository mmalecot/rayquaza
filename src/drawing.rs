//! Drawing utilities.

use crate::{camera::Camera2D, color::Color, ffi, window::Window};
use std::marker::PhantomData;

/// Container that holds various drawing elements.
pub struct Canvas(PhantomData<*const ()>);

impl Canvas {
    /// Sets background color.
    #[inline]
    pub fn clear_background(&mut self, color: impl Into<Color>) {
        unsafe { ffi::ClearBackground(color.into().into()) }
    }

    /// Draws in 2D mode with custom camera.
    pub fn mode_2d<F>(&mut self, camera: Camera2D, function: F)
    where
        F: FnOnce(&mut Canvas),
    {
        unsafe {
            ffi::BeginMode2D(camera.into());
            function(self);
            ffi::EndMode2D();
        }
    }
}

/// Drawing.
impl Window {
    /// Draws in a canvas and swap buffers (double buffering).
    pub fn draw<F>(&self, function: F)
    where
        F: FnOnce(&mut Canvas),
    {
        unsafe {
            ffi::BeginDrawing();
            function(&mut Canvas(PhantomData));
            ffi::EndDrawing();
        }
    }
}
