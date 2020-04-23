//! Timing module.

use crate::{ffi, Window};

/// Timing.
impl Window {
    /// Sets target FPS (maximum).
    pub fn set_target_fps(&mut self, fps: i32) {
        unsafe { ffi::SetTargetFPS(fps) }
    }

    /// Returns current FPS.
    pub fn get_fps(&self) -> i32 {
        unsafe { ffi::GetFPS() }
    }

    /// Returns time in seconds for last frame drawn.
    pub fn get_frame_time(&self) -> f32 {
        unsafe { ffi::GetFrameTime() }
    }

    /// Returns elapsed time in seconds since window creation.
    pub fn get_time(&self) -> f64 {
        unsafe { ffi::GetTime() }
    }
}
