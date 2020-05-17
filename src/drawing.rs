//! Utilities for drawing.

use crate::{camera::Camera2D, color::Color, ffi, window::Window};
use std::marker::PhantomData;

/// Canvas.
pub struct Canvas(PhantomData<*const ()>);

impl Canvas {
    /// Creates an new canvas to start drawing.
    fn new() -> Canvas {
        unsafe {
            ffi::BeginDrawing();
            Canvas(PhantomData)
        }
    }

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

impl Drop for Canvas {
    fn drop(&mut self) {
        unsafe {
            ffi::EndDrawing();
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
        function(&mut Canvas::new())
    }
}
