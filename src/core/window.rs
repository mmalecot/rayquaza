//! Window module.

use crate::{
    core::{
        drawing::Canvas,
        error::Error,
        input::{Button, Key},
        math::Vector2,
    },
    ffi,
};
use std::ffi::{CStr, CString};

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

    /// Checks if Escape key pressed or Close icon pressed.
    pub fn should_close(&self) -> bool {
        unsafe { ffi::WindowShouldClose() }
    }

    /// Checks if window has been minimized (or lost focus).
    pub fn is_minimized(&self) -> bool {
        unsafe { ffi::IsWindowMinimized() }
    }

    /// Checks if window has been resized.
    pub fn is_resized(&self) -> bool {
        unsafe { ffi::IsWindowResized() }
    }

    /// Checks if window is currently hidden.
    pub fn is_hidden(&self) -> bool {
        unsafe { ffi::IsWindowHidden() }
    }

    /// Checks if window is currently fullscreen.
    pub fn is_fullscreen(&self) -> bool {
        unsafe { ffi::IsWindowFullscreen() }
    }

    /// Toggles fullscreen mode.
    pub fn toggle_fullscreen(&mut self) {
        unsafe { ffi::ToggleFullscreen() }
    }

    /// Shows the window.
    pub fn show(&mut self) {
        unsafe { ffi::UnhideWindow() }
    }

    /// Hides the window.
    pub fn hide(&mut self) {
        unsafe { ffi::HideWindow() }
    }

    /// Sets title for window.
    pub fn set_title(&mut self, title: &str) {
        unsafe {
            let title = CString::new(title).unwrap();
            ffi::SetWindowTitle(title.as_ptr())
        }
    }

    /// Sets window position on screen.
    pub fn set_position(&mut self, x: i32, y: i32) {
        unsafe { ffi::SetWindowPosition(x, y) }
    }

    /// Sets monitor for the current window.
    pub fn set_monitor(&mut self, monitor: i32) {
        unsafe { ffi::SetWindowMonitor(monitor) }
    }

    /// Sets window minimum dimensions.
    pub fn set_min_size(&mut self, width: i32, height: i32) {
        unsafe { ffi::SetWindowMinSize(width, height) }
    }

    /// Sets window dimensions.
    pub fn set_size(&mut self, width: i32, height: i32) {
        unsafe { ffi::SetWindowSize(width, height) }
    }

    /// Gets current screen width.
    pub fn get_screen_width(&self) -> i32 {
        unsafe { ffi::GetScreenWidth() }
    }

    /// Gets current screen height.
    pub fn get_screen_height(&self) -> i32 {
        unsafe { ffi::GetScreenHeight() }
    }

    /// Gets number of connected monitors.
    pub fn get_monitor_count(&self) -> i32 {
        unsafe { ffi::GetMonitorCount() }
    }

    /// Gets monitor width.
    pub fn get_monitor_width(&self, monitor: i32) -> i32 {
        unsafe { ffi::GetMonitorWidth(monitor) }
    }

    /// Gest monitor height.
    pub fn get_monitor_height(&self, monitor: i32) -> i32 {
        unsafe { ffi::GetMonitorHeight(monitor) }
    }

    /// Gets monitor physical width in millimetres.
    pub fn get_monitor_physical_width(&self, monitor: i32) -> i32 {
        unsafe { ffi::GetMonitorPhysicalWidth(monitor) }
    }

    /// Gets monitor physical height in millimetres
    pub fn get_monitor_physical_height(&self, monitor: i32) -> i32 {
        unsafe { ffi::GetMonitorPhysicalHeight(monitor) }
    }

    /// Gets window position XY on monitor.
    pub fn get_position(&self) -> Vector2 {
        unsafe { ffi::GetWindowPosition().into() }
    }

    /// Gets the human-readable name of monitor.
    pub fn get_monitor_name(&self, monitor: i32) -> String {
        unsafe {
            CStr::from_ptr(ffi::GetMonitorName(monitor))
                .to_str()
                .unwrap()
                .to_string()
        }
    }

    /// Gets clipboard text content.
    pub fn get_clipboard(&self) -> String {
        unsafe {
            CStr::from_ptr(ffi::GetClipboardText())
                .to_str()
                .unwrap()
                .to_string()
        }
    }

    /// Sets clipboard text content.
    pub fn set_clipboard(&mut self, text: &str) {
        unsafe {
            let text = CString::new(text).unwrap();
            ffi::SetClipboardText(text.as_ptr());
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

    /// Detects if a key is being pressed.
    pub fn is_key_down(&self, key: Key) -> bool {
        unsafe { ffi::IsKeyDown(key as i32) }
    }

    /// Detect if a mouse button has been pressed once.
    pub fn is_mouse_button_pressed(&self, button: Button) -> bool {
        unsafe { ffi::IsMouseButtonPressed(button as i32) }
    }

    /// Detects if a mouse button is being pressed.
    pub fn is_mouse_button_down(&self, button: Button) -> bool {
        unsafe { ffi::IsMouseButtonDown(button as i32) }
    }

    /// Detects if a mouse button has been released once.
    pub fn is_mouse_button_released(&self, button: Button) -> bool {
        unsafe { ffi::IsMouseButtonReleased(button as i32) }
    }

    /// Detects if a mouse button is NOT being pressed.
    pub fn is_mouse_button_up(&self, button: Button) -> bool {
        unsafe { ffi::IsMouseButtonUp(button as i32) }
    }

    /// Returns mouse position X.
    pub fn get_mouse_x(&self) -> i32 {
        unsafe { ffi::GetMouseX() }
    }

    /// Returns mouse position Y.
    pub fn get_mouse_y(&self) -> i32 {
        unsafe { ffi::GetMouseY() }
    }

    /// Returns mouse position XY.
    pub fn get_mouse_position(&self) -> Vector2 {
        unsafe { ffi::GetMousePosition().into() }
    }

    /// Sets mouse position XY.
    pub fn set_mouse_position(&mut self, x: i32, y: i32) {
        unsafe { ffi::SetMousePosition(x, y) }
    }

    /// Sets mouse offset.
    pub fn set_mouse_offset(&mut self, x: i32, y: i32) {
        unsafe { ffi::SetMouseOffset(x, y) }
    }

    /// Sets mouse scaling.
    pub fn set_mouse_scale(&mut self, x: f32, y: f32) {
        unsafe { ffi::SetMouseScale(x, y) }
    }

    /// Returns mouse wheel movement Y.
    pub fn get_mouse_wheel_move(&self) -> i32 {
        unsafe { ffi::GetMouseWheelMove() }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe { ffi::CloseWindow() }
    }
}
