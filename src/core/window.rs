//! Window module.

use crate::{
    core::{drawing::Canvas, error::Error, input::Key},
    ffi,
};
use std::ffi::CString;

/// Window type.
pub struct Window;

impl Window {
    /// Initializes a window and an OpenGL context.
    pub fn create(width: i32, height: i32, title: &str) -> Result<Window, Error> {
        unsafe {
            let title = CString::new(title).unwrap();
            ffi::InitWindow(width, height, title.as_ptr());
            if ffi::IsWindowReady() {
                Ok(Window)
            } else {
                Err(Error::InitWindowFailed)
            }
        }
    }

    /// Sets target FPS (maximum).
    pub fn set_target_fps(&mut self, fps: i32) {
        unsafe { ffi::SetTargetFPS(fps) }
    }

    /// Draws in a canvas and swap buffers (double buffering).
    pub fn draw<F>(&mut self, function: F)
    where
        F: FnOnce(&mut Canvas),
    {
        function(&mut Canvas::new())
    }

    /// Checks if Escape key pressed or Close icon pressed.
    pub fn should_close(&self) -> bool {
        unsafe { ffi::WindowShouldClose() }
    }

    /// Detects if a key is being pressed.
    pub fn is_key_down(&self, key: Key) -> bool {
        unsafe { ffi::IsKeyDown(key as i32) }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            ffi::CloseWindow();
        }
    }
}
