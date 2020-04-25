//! Cursor module.

use crate::{ffi, window::Window};

/// Cursor.
impl Window {
    /// Shows cursor.
    pub fn show_cursor(&mut self) {
        unsafe {
            ffi::ShowCursor();
        }
    }

    /// Hides cursor.
    pub fn hide_cursor(&mut self) {
        unsafe {
            ffi::HideCursor();
        }
    }

    /// Checks if cursor is not visible.
    pub fn is_cursor_hidden(&mut self) -> bool {
        unsafe { ffi::IsCursorHidden() }
    }

    /// Enables cursor.
    pub fn enable_cursor(&mut self) {
        unsafe {
            ffi::EnableCursor();
        }
    }

    /// Disables cursor.
    pub fn disable_cursor(&mut self) {
        unsafe {
            ffi::DisableCursor();
        }
    }
}
