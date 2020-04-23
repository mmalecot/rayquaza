//! Window module.

use crate::{ffi, math::Vector2, CreateWindowError};
use std::{
    ffi::{CStr, CString},
    rc::Rc,
    sync::atomic::{AtomicBool, Ordering},
};

/// Holder of the window.
pub(crate) struct Handle;

impl Drop for Handle {
    fn drop(&mut self) {
        unsafe {
            ffi::CloseWindow();
        }
    }
}

/// Window.
pub struct Window {
    pub(crate) handle: Rc<Handle>,
}

impl Window {
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

    /// Gets monitor height.
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
}

/// Window builder.
pub struct WindowBuilder {
    flags: u32,
    width: i32,
    height: i32,
    title: String,
}

impl WindowBuilder {
    /// Creates a new `WindowBuilder`.
    pub fn new() -> WindowBuilder {
        WindowBuilder::default()
    }

    /// Sets window size.
    pub fn size(mut self, width: i32, height: i32) -> WindowBuilder {
        self.width = width;
        self.height = height;
        self
    }

    /// Sets window title.
    pub fn title(mut self, title: &str) -> WindowBuilder {
        self.title = title.to_string();
        self
    }

    /// Sets to run window in fullscreen.
    pub fn fullscreen(mut self) -> WindowBuilder {
        self.flags |= ffi::FLAG_FULLSCREEN_MODE;
        self.flags &= !ffi::FLAG_WINDOW_RESIZABLE;
        self
    }

    /// Sets to allow resizable window.
    pub fn resizable(mut self) -> WindowBuilder {
        self.flags |= ffi::FLAG_WINDOW_RESIZABLE;
        self.flags &= !ffi::FLAG_FULLSCREEN_MODE;
        self
    }

    /// Sets to disable window decoration.
    pub fn undecorated(mut self) -> WindowBuilder {
        self.flags |= ffi::FLAG_WINDOW_UNDECORATED;
        self
    }

    /// Sets to allow transparent window.
    pub fn transparent(mut self) -> WindowBuilder {
        self.flags |= ffi::FLAG_WINDOW_TRANSPARENT;
        self
    }

    /// Sets to create the window initially hidden.
    pub fn hidden(mut self) -> WindowBuilder {
        self.flags |= ffi::FLAG_WINDOW_HIDDEN;
        self
    }

    /// Sets to allow window running while minimized.
    pub fn always_run(mut self) -> WindowBuilder {
        self.flags |= ffi::FLAG_WINDOW_ALWAYS_RUN;
        self
    }

    /// Sets to try enabling MSAA 4X.
    pub fn msaa_4x(mut self) -> WindowBuilder {
        self.flags |= ffi::FLAG_MSAA_4X_HINT;
        self
    }

    /// Sets to try enabling V-Sync on GPU.
    pub fn vsync(mut self) -> WindowBuilder {
        self.flags |= ffi::FLAG_VSYNC_HINT;
        self
    }

    /// Builds the window.
    pub fn build(self) -> Result<Window, CreateWindowError> {
        unsafe {
            static INITIALIZED: AtomicBool = AtomicBool::new(false);
            if INITIALIZED.load(Ordering::Relaxed) {
                Err(CreateWindowError::AlreadyCreated)
            } else {
                ffi::SetConfigFlags(self.flags);
                let title = CString::new(self.title).unwrap();
                ffi::InitWindow(self.width, self.height, title.as_ptr());
                if ffi::IsWindowReady() {
                    INITIALIZED.store(true, Ordering::Relaxed);
                    Ok(Window {
                        handle: Rc::new(Handle),
                    })
                } else {
                    Err(CreateWindowError::InitializationFailed)
                }
            }
        }
    }
}

impl Default for WindowBuilder {
    fn default() -> WindowBuilder {
        WindowBuilder {
            flags: Default::default(),
            width: 1280,
            height: 720,
            title: String::from(concat!(
                env!("CARGO_PKG_NAME"),
                " ",
                env!("CARGO_PKG_VERSION")
            )),
        }
    }
}
