//! Timing module.

use crate::{ffi, Window};

/// Timing.
impl Window {
    /// Sets target FPS (maximum).
    #[inline]
    pub fn set_target_fps(&mut self, fps: i32) {
        unsafe { ffi::SetTargetFPS(fps) }
    }

    /// Returns current FPS.
    #[inline]
    pub fn get_fps(&self) -> i32 {
        unsafe { ffi::GetFPS() }
    }

    /// Returns time in seconds for last frame drawn.
    #[inline]
    pub fn get_frame_time(&self) -> f32 {
        unsafe { ffi::GetFrameTime() }
    }

    /// Returns elapsed time in seconds since window creation.
    #[inline]
    pub fn get_time(&self) -> f64 {
        unsafe { ffi::GetTime() }
    }
}
