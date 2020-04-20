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

    /// Draws in a canvas and swap buffers (double buffering).
    pub fn draw<F>(&mut self, function: F)
    where
        F: FnOnce(&mut Canvas),
    {
        function(&mut Canvas::new())
    }

    /// Detects if a key has been pressed once.
    pub fn is_key_pressed(&self, key: Key) -> bool {
        unsafe { ffi::IsKeyPressed(key as i32) }
    }

    /// Detects if a key is being pressed.
    pub fn is_key_down(&self, key: Key) -> bool {
        unsafe { ffi::IsKeyDown(key as i32) }
    }

    /// Detects if a key has been released once.
    pub fn is_key_released(&self, key: Key) -> bool {
        unsafe { ffi::IsKeyReleased(key as i32) }
    }

    /// Detects if a key is NOT being pressed
    pub fn is_key_up(&self, key: Key) -> bool {
        unsafe { ffi::IsKeyUp(key as i32) }
    }

    /// Gets latest key pressed.
    pub fn get_key_pressed(&self) -> Option<Key> {
        unsafe {
            match ffi::GetKeyPressed() {
                ffi::KEY_APOSTROPHE => Some(Key::Apostrophe),
                ffi::KEY_COMMA => Some(Key::Comma),
                ffi::KEY_MINUS => Some(Key::Minus),
                ffi::KEY_PERIOD => Some(Key::Period),
                ffi::KEY_SLASH => Some(Key::Slash),
                ffi::KEY_ZERO => Some(Key::Zero),
                ffi::KEY_ONE => Some(Key::One),
                ffi::KEY_TWO => Some(Key::Two),
                ffi::KEY_THREE => Some(Key::Three),
                ffi::KEY_FOUR => Some(Key::Four),
                ffi::KEY_FIVE => Some(Key::Five),
                ffi::KEY_SIX => Some(Key::Six),
                ffi::KEY_SEVEN => Some(Key::Seven),
                ffi::KEY_EIGHT => Some(Key::Eight),
                ffi::KEY_NINE => Some(Key::Nine),
                ffi::KEY_SEMICOLON => Some(Key::Semicolon),
                ffi::KEY_EQUAL => Some(Key::Equal),
                ffi::KEY_A => Some(Key::A),
                ffi::KEY_B => Some(Key::B),
                ffi::KEY_C => Some(Key::C),
                ffi::KEY_D => Some(Key::D),
                ffi::KEY_E => Some(Key::E),
                ffi::KEY_F => Some(Key::F),
                ffi::KEY_G => Some(Key::G),
                ffi::KEY_H => Some(Key::H),
                ffi::KEY_I => Some(Key::I),
                ffi::KEY_J => Some(Key::J),
                ffi::KEY_K => Some(Key::K),
                ffi::KEY_L => Some(Key::L),
                ffi::KEY_M => Some(Key::M),
                ffi::KEY_N => Some(Key::N),
                ffi::KEY_O => Some(Key::O),
                ffi::KEY_P => Some(Key::P),
                ffi::KEY_Q => Some(Key::Q),
                ffi::KEY_R => Some(Key::R),
                ffi::KEY_S => Some(Key::S),
                ffi::KEY_T => Some(Key::T),
                ffi::KEY_U => Some(Key::U),
                ffi::KEY_V => Some(Key::V),
                ffi::KEY_W => Some(Key::W),
                ffi::KEY_X => Some(Key::X),
                ffi::KEY_Y => Some(Key::Y),
                ffi::KEY_Z => Some(Key::Z),
                ffi::KEY_SPACE => Some(Key::Space),
                ffi::KEY_ESCAPE => Some(Key::Escape),
                ffi::KEY_ENTER => Some(Key::Enter),
                ffi::KEY_TAB => Some(Key::Tab),
                ffi::KEY_BACKSPACE => Some(Key::Backspace),
                ffi::KEY_INSERT => Some(Key::Insert),
                ffi::KEY_DELETE => Some(Key::Delete),
                ffi::KEY_RIGHT => Some(Key::Right),
                ffi::KEY_LEFT => Some(Key::Left),
                ffi::KEY_DOWN => Some(Key::Down),
                ffi::KEY_UP => Some(Key::Up),
                ffi::KEY_PAGE_UP => Some(Key::PageUp),
                ffi::KEY_PAGE_DOWN => Some(Key::PageDown),
                ffi::KEY_HOME => Some(Key::Home),
                ffi::KEY_END => Some(Key::End),
                ffi::KEY_CAPS_LOCK => Some(Key::CapsLock),
                ffi::KEY_SCROLL_LOCK => Some(Key::ScrollLock),
                ffi::KEY_NUM_LOCK => Some(Key::NumLock),
                ffi::KEY_PRINT_SCREEN => Some(Key::PrintScreen),
                ffi::KEY_PAUSE => Some(Key::Pause),
                ffi::KEY_F1 => Some(Key::F1),
                ffi::KEY_F2 => Some(Key::F2),
                ffi::KEY_F3 => Some(Key::F3),
                ffi::KEY_F4 => Some(Key::F4),
                ffi::KEY_F5 => Some(Key::F5),
                ffi::KEY_F6 => Some(Key::F6),
                ffi::KEY_F7 => Some(Key::F7),
                ffi::KEY_F8 => Some(Key::F8),
                ffi::KEY_F9 => Some(Key::F9),
                ffi::KEY_F10 => Some(Key::F10),
                ffi::KEY_F11 => Some(Key::F11),
                ffi::KEY_F12 => Some(Key::F12),
                ffi::KEY_LEFT_SHIFT => Some(Key::LeftShift),
                ffi::KEY_LEFT_CONTROL => Some(Key::LeftControl),
                ffi::KEY_LEFT_ALT => Some(Key::LeftAlt),
                ffi::KEY_LEFT_SUPER => Some(Key::LeftSuper),
                ffi::KEY_RIGHT_SHIFT => Some(Key::RightShift),
                ffi::KEY_RIGHT_CONTROL => Some(Key::RightControl),
                ffi::KEY_RIGHT_ALT => Some(Key::RightAlt),
                ffi::KEY_RIGHT_SUPER => Some(Key::RightSuper),
                ffi::KEY_KB_MENU => Some(Key::Menu),
                ffi::KEY_LEFT_BRACKET => Some(Key::LeftBracket),
                ffi::KEY_BACKSLASH => Some(Key::Backslash),
                ffi::KEY_RIGHT_BRACKET => Some(Key::RightBracket),
                ffi::KEY_GRAVE => Some(Key::Grave),
                ffi::KEY_KP_0 => Some(Key::KeypadZero),
                ffi::KEY_KP_1 => Some(Key::KeypadOne),
                ffi::KEY_KP_2 => Some(Key::KeypadTwo),
                ffi::KEY_KP_3 => Some(Key::KeypadThree),
                ffi::KEY_KP_4 => Some(Key::KeypadFour),
                ffi::KEY_KP_5 => Some(Key::KeypadFive),
                ffi::KEY_KP_6 => Some(Key::KeypadSix),
                ffi::KEY_KP_7 => Some(Key::KeypadSeven),
                ffi::KEY_KP_8 => Some(Key::KeypadEight),
                ffi::KEY_KP_9 => Some(Key::KeypadNine),
                ffi::KEY_KP_DECIMAL => Some(Key::KeypadDecimal),
                ffi::KEY_KP_DIVIDE => Some(Key::KeypadDivide),
                ffi::KEY_KP_MULTIPLY => Some(Key::KeypadMultiply),
                ffi::KEY_KP_SUBTRACT => Some(Key::KeypadSubtract),
                ffi::KEY_KP_ADD => Some(Key::KeypadAdd),
                ffi::KEY_KP_ENTER => Some(Key::KeypadEnter),
                ffi::KEY_KP_EQUAL => Some(Key::KeypadEqual),
                _ => None,
            }
        }
    }

    /// Sets a custom key to exit program (default is ESC).
    pub fn set_exit_key(&mut self, key: Key) {
        unsafe { ffi::SetExitKey(key as i32) }
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
