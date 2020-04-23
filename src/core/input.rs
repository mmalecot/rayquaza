//! Input module.

use crate::ffi;

/// Kinds of mouse buttons.
#[repr(i32)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MouseButton {
    Left = ffi::MOUSE_LEFT_BUTTON,
    Right = ffi::MOUSE_RIGHT_BUTTON,
    Middle = ffi::MOUSE_MIDDLE_BUTTON,
}

/// Gamepad number.
#[repr(i32)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Gamepad {
    One = ffi::GAMEPAD_PLAYER1,
    Two = ffi::GAMEPAD_PLAYER2,
    Three = ffi::GAMEPAD_PLAYER3,
    Four = ffi::GAMEPAD_PLAYER4,
}

/// Kinds of gamepad axis.
#[repr(C)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GamepadAxis {
    // Left stick
    LeftX = 1,
    LeftY,

    // Right stick
    RightX,
    RightY,

    // Pressure levels for the back triggers
    LeftTrigger,
    RightTrigger,
}

/// Kinds of gamepad buttons.
#[repr(C)]
#[derive(Clone, Debug, Eq, PartialEq)]
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
#[derive(Clone, Debug, Eq, PartialEq)]
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
