use crate::{
    core::{drawing::Drawing, error::Error},
    ffi,
};
use std::ffi::CString;

pub struct Window;

impl Window {
    /// Initialize window and OpenGL context.
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

    /// Set target FPS (maximum).
    pub fn set_target_fps(&mut self, fps: i32) {
        unsafe { ffi::SetTargetFPS(fps) }
    }

    /// Setup canvas (framebuffer) to start drawing.
    pub fn drawing(&mut self) -> Drawing {
        Drawing::new()
    }

    /// Check if KEY_ESCAPE pressed or Close icon pressed.
    pub fn should_close(&self) -> bool {
        unsafe { ffi::WindowShouldClose() }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            ffi::CloseWindow();
        }
    }
}
