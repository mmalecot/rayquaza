//! Input management.

use crate::{ffi, math::Vector2, window::Window};
use std::{
    ffi::{CStr, CString},
    mem,
};

/// Kinds of mouse buttons.
#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MouseButton {
    Left = ffi::MOUSE_LEFT_BUTTON,
    Right = ffi::MOUSE_RIGHT_BUTTON,
    Middle = ffi::MOUSE_MIDDLE_BUTTON,
}

/// Gamepad number.
#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Gamepad {
    One = ffi::GAMEPAD_PLAYER1,
    Two = ffi::GAMEPAD_PLAYER2,
    Three = ffi::GAMEPAD_PLAYER3,
    Four = ffi::GAMEPAD_PLAYER4,
}

/// Kinds of gamepad axis.
#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GamepadAxis {
    // Left stick
    LeftX = ffi::GAMEPAD_AXIS_LEFT_X,
    LeftY = ffi::GAMEPAD_AXIS_LEFT_Y,

    // Right stick
    RightX = ffi::GAMEPAD_AXIS_RIGHT_X,
    RightY = ffi::GAMEPAD_AXIS_RIGHT_Y,

    // Pressure levels for the back triggers
    LeftTrigger = ffi::GAMEPAD_AXIS_LEFT_TRIGGER,
    RightTrigger = ffi::GAMEPAD_AXIS_RIGHT_TRIGGER,
}

/// Kinds of gamepad buttons.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GamepadButton {
    // Left buttons (normally D-Pad)
    LeftFaceUp = 1,
    LeftFaceRight,
    LeftFaceDown,
    LeftFaceLeft,

    // Right buttons
    RightFaceUp,
    RightFaceRight,
    RightFaceDown,
    RightFaceLeft,

    // Triggers
    LeftTrigger1,
    LeftTrigger2,
    RightTrigger1,
    RightTrigger2,

    // Middle buttons
    MiddleLeft,
    Middle,
    MiddleRight,

    // Joystick press in buttons
    MiddleLeftThumb,
    MiddleRightThumb,
}

/// Kinds of keyboard keys.
#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Key {
    // Alphanumeric keys
    Apostrophe = ffi::KEY_APOSTROPHE,
    Comma = ffi::KEY_COMMA,
    Minus = ffi::KEY_MINUS,
    Period = ffi::KEY_PERIOD,
    Slash = ffi::KEY_SLASH,
    Zero = ffi::KEY_ZERO,
    One = ffi::KEY_ONE,
    Two = ffi::KEY_TWO,
    Three = ffi::KEY_THREE,
    Four = ffi::KEY_FOUR,
    Five = ffi::KEY_FIVE,
    Six = ffi::KEY_SIX,
    Seven = ffi::KEY_SEVEN,
    Eight = ffi::KEY_EIGHT,
    Nine = ffi::KEY_NINE,
    Semicolon = ffi::KEY_SEMICOLON,
    Equal = ffi::KEY_EQUAL,
    A = ffi::KEY_A,
    B = ffi::KEY_B,
    C = ffi::KEY_C,
    D = ffi::KEY_D,
    E = ffi::KEY_E,
    F = ffi::KEY_F,
    G = ffi::KEY_G,
    H = ffi::KEY_H,
    I = ffi::KEY_I,
    J = ffi::KEY_J,
    K = ffi::KEY_K,
    L = ffi::KEY_L,
    M = ffi::KEY_M,
    N = ffi::KEY_N,
    O = ffi::KEY_O,
    P = ffi::KEY_P,
    Q = ffi::KEY_Q,
    R = ffi::KEY_R,
    S = ffi::KEY_S,
    T = ffi::KEY_T,
    U = ffi::KEY_U,
    V = ffi::KEY_V,
    W = ffi::KEY_W,
    X = ffi::KEY_X,
    Y = ffi::KEY_Y,
    Z = ffi::KEY_Z,

    // Function keys
    Space = ffi::KEY_SPACE,
    Escape = ffi::KEY_ESCAPE,
    Enter = ffi::KEY_ENTER,
    Tab = ffi::KEY_TAB,
    Backspace = ffi::KEY_BACKSPACE,
    Insert = ffi::KEY_INSERT,
    Delete = ffi::KEY_DELETE,
    Right = ffi::KEY_RIGHT,
    Left = ffi::KEY_LEFT,
    Down = ffi::KEY_DOWN,
    Up = ffi::KEY_UP,
    PageUp = ffi::KEY_PAGE_UP,
    PageDown = ffi::KEY_PAGE_DOWN,
    Home = ffi::KEY_HOME,
    End = ffi::KEY_END,
    CapsLock = ffi::KEY_CAPS_LOCK,
    ScrollLock = ffi::KEY_SCROLL_LOCK,
    NumLock = ffi::KEY_NUM_LOCK,
    PrintScreen = ffi::KEY_PRINT_SCREEN,
    Pause = ffi::KEY_PAUSE,
    F1 = ffi::KEY_F1,
    F2 = ffi::KEY_F2,
    F3 = ffi::KEY_F3,
    F4 = ffi::KEY_F4,
    F5 = ffi::KEY_F5,
    F6 = ffi::KEY_F6,
    F7 = ffi::KEY_F7,
    F8 = ffi::KEY_F8,
    F9 = ffi::KEY_F9,
    F10 = ffi::KEY_F10,
    F11 = ffi::KEY_F11,
    F12 = ffi::KEY_F12,
    LeftShift = ffi::KEY_LEFT_SHIFT,
    LeftControl = ffi::KEY_LEFT_CONTROL,
    LeftAlt = ffi::KEY_LEFT_ALT,
    LeftSuper = ffi::KEY_LEFT_SUPER,
    RightShift = ffi::KEY_RIGHT_SHIFT,
    RightControl = ffi::KEY_RIGHT_CONTROL,
    RightAlt = ffi::KEY_RIGHT_ALT,
    RightSuper = ffi::KEY_RIGHT_SUPER,
    Menu = ffi::KEY_KB_MENU,
    LeftBracket = ffi::KEY_LEFT_BRACKET,
    Backslash = ffi::KEY_BACKSLASH,
    RightBracket = ffi::KEY_RIGHT_BRACKET,
    Grave = ffi::KEY_GRAVE,

    // Keypad key
    KeypadZero = ffi::KEY_KP_0,
    KeypadOne = ffi::KEY_KP_1,
    KeypadTwo = ffi::KEY_KP_2,
    KeypadThree = ffi::KEY_KP_3,
    KeypadFour = ffi::KEY_KP_4,
    KeypadFive = ffi::KEY_KP_5,
    KeypadSix = ffi::KEY_KP_6,
    KeypadSeven = ffi::KEY_KP_7,
    KeypadEight = ffi::KEY_KP_8,
    KeypadNine = ffi::KEY_KP_9,
    KeypadDecimal = ffi::KEY_KP_DECIMAL,
    KeypadDivide = ffi::KEY_KP_DIVIDE,
    KeypadMultiply = ffi::KEY_KP_MULTIPLY,
    KeypadSubtract = ffi::KEY_KP_SUBTRACT,
    KeypadAdd = ffi::KEY_KP_ADD,
    KeypadEnter = ffi::KEY_KP_ENTER,
    KeypadEqual = ffi::KEY_KP_EQUAL,
}

/// Input.
impl Window {
    /// Detects if a key has been pressed once.
    #[inline]
    pub fn is_key_pressed(&self, key: Key) -> bool {
        unsafe { ffi::IsKeyPressed(key as i32) }
    }

    /// Detects if a key is being pressed.
    #[inline]
    pub fn is_key_down(&self, key: Key) -> bool {
        unsafe { ffi::IsKeyDown(key as i32) }
    }

    /// Detects if a key has been released once.
    #[inline]
    pub fn is_key_released(&self, key: Key) -> bool {
        unsafe { ffi::IsKeyReleased(key as i32) }
    }

    /// Detects if a key is NOT being pressed
    #[inline]
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
    #[inline]
    pub fn set_exit_key(&mut self, key: Key) {
        unsafe { ffi::SetExitKey(key as i32) }
    }

    /// Detect if a mouse button has been pressed once.
    #[inline]
    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        unsafe { ffi::IsMouseButtonPressed(button as i32) }
    }

    /// Detects if a mouse button is being pressed.
    #[inline]
    pub fn is_mouse_button_down(&self, button: MouseButton) -> bool {
        unsafe { ffi::IsMouseButtonDown(button as i32) }
    }

    /// Detects if a mouse button has been released once.
    #[inline]
    pub fn is_mouse_button_released(&self, button: MouseButton) -> bool {
        unsafe { ffi::IsMouseButtonReleased(button as i32) }
    }

    /// Detects if a mouse button is NOT being pressed.
    #[inline]
    pub fn is_mouse_button_up(&self, button: MouseButton) -> bool {
        unsafe { ffi::IsMouseButtonUp(button as i32) }
    }

    /// Returns mouse position X.
    #[inline]
    pub fn get_mouse_x(&self) -> i32 {
        unsafe { ffi::GetMouseX() }
    }

    /// Returns mouse position Y.
    #[inline]
    pub fn get_mouse_y(&self) -> i32 {
        unsafe { ffi::GetMouseY() }
    }

    /// Returns mouse position XY.
    #[inline]
    pub fn get_mouse_position(&self) -> Vector2 {
        unsafe { ffi::GetMousePosition().into() }
    }

    /// Sets mouse position XY.
    #[inline]
    pub fn set_mouse_position(&mut self, x: i32, y: i32) {
        unsafe { ffi::SetMousePosition(x, y) }
    }

    /// Sets mouse offset.
    #[inline]
    pub fn set_mouse_offset(&mut self, x: i32, y: i32) {
        unsafe { ffi::SetMouseOffset(x, y) }
    }

    /// Sets mouse scaling.
    #[inline]
    pub fn set_mouse_scale(&mut self, x: f32, y: f32) {
        unsafe { ffi::SetMouseScale(x, y) }
    }

    /// Returns mouse wheel movement Y.
    #[inline]
    pub fn get_mouse_wheel_move(&self) -> i32 {
        unsafe { ffi::GetMouseWheelMove() }
    }

    /// Detects if a gamepad is available.
    #[inline]
    pub fn is_gamepad_available(&self, gamepad: Gamepad) -> bool {
        unsafe { ffi::IsGamepadAvailable(gamepad as i32) }
    }

    /// Checks gamepad name (if available).
    pub fn is_gamepad_name(&self, gamepad: Gamepad, name: &str) -> bool {
        unsafe {
            let name = CString::new(name).unwrap();
            ffi::IsGamepadName(gamepad as i32, name.as_ptr())
        }
    }

    /// Returns gamepad internal name id.
    pub fn get_gamepad_name(&self, gamepad: Gamepad) -> Option<String> {
        unsafe {
            let name = ffi::GetGamepadName(gamepad as i32);
            if name.is_null() {
                None
            } else {
                Some(CStr::from_ptr(name).to_str().unwrap().to_string())
            }
        }
    }

    /// Detects if a gamepad button has been pressed once.
    #[inline]
    pub fn is_gamepad_button_pressed(&self, gamepad: Gamepad, button: GamepadButton) -> bool {
        unsafe { ffi::IsGamepadButtonPressed(gamepad as i32, button as i32) }
    }
    /// Detects if a gamepad button is being pressed.
    pub fn is_gamepad_button_down(&self, gamepad: Gamepad, button: GamepadButton) -> bool {
        unsafe { ffi::IsGamepadButtonDown(gamepad as i32, button as i32) }
    }

    /// Detects if a gamepad button has been released once.
    #[inline]
    pub fn is_gamepad_button_released(&self, gamepad: Gamepad, button: GamepadButton) -> bool {
        unsafe { ffi::IsGamepadButtonReleased(gamepad as i32, button as i32) }
    }

    /// Detects if a gamepad button is NOT being pressed.
    #[inline]
    pub fn is_gamepad_button_up(&self, gamepad: Gamepad, button: GamepadButton) -> bool {
        unsafe { ffi::IsGamepadButtonUp(gamepad as i32, button as i32) }
    }

    /// Gets the last gamepad button pressed.
    pub fn get_gamepad_button_pressed(&self) -> Option<GamepadButton> {
        unsafe {
            let button = ffi::GetGamepadButtonPressed();
            if button > 0 {
                Some(mem::transmute(button))
            } else {
                None
            }
        }
    }

    /// Returns gamepad axis count for a gamepad.
    #[inline]
    pub fn get_gamepad_axis_count(&self, gamepad: Gamepad) -> i32 {
        unsafe { ffi::GetGamepadAxisCount(gamepad as i32) }
    }

    /// Returns axis movement value for a gamepad axis.
    #[inline]
    pub fn get_gamepad_axis_movement(&self, gamepad: Gamepad, axis: GamepadAxis) -> f32 {
        unsafe { ffi::GetGamepadAxisMovement(gamepad as i32, axis as i32) }
    }
}
