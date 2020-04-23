//! Drawing module.

use crate::{color::Color, ffi, Window};
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
    pub fn clear_background(&mut self, color: impl Into<Color>) {
        unsafe { ffi::ClearBackground(color.into().into()) }
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
