//! raylib raw FFI bindings.

use std::os::raw::{c_char, c_float, c_int, c_uchar};

// Constants
// Alphanumeric keys
pub const KEY_APOSTROPHE: i32 = 39;
pub const KEY_COMMA: i32 = 44;
pub const KEY_MINUS: i32 = 45;
pub const KEY_PERIOD: i32 = 46;
pub const KEY_SLASH: i32 = 47;
pub const KEY_ZERO: i32 = 48;
pub const KEY_ONE: i32 = 49;
pub const KEY_TWO: i32 = 50;
pub const KEY_THREE: i32 = 51;
pub const KEY_FOUR: i32 = 52;
pub const KEY_FIVE: i32 = 53;
pub const KEY_SIX: i32 = 54;
pub const KEY_SEVEN: i32 = 55;
pub const KEY_EIGHT: i32 = 56;
pub const KEY_NINE: i32 = 57;
pub const KEY_SEMICOLON: i32 = 59;
pub const KEY_EQUAL: i32 = 61;
pub const KEY_A: i32 = 65;
pub const KEY_B: i32 = 66;
pub const KEY_C: i32 = 67;
pub const KEY_D: i32 = 68;
pub const KEY_E: i32 = 69;
pub const KEY_F: i32 = 70;
pub const KEY_G: i32 = 71;
pub const KEY_H: i32 = 72;
pub const KEY_I: i32 = 73;
pub const KEY_J: i32 = 74;
pub const KEY_K: i32 = 75;
pub const KEY_L: i32 = 76;
pub const KEY_M: i32 = 77;
pub const KEY_N: i32 = 78;
pub const KEY_O: i32 = 79;
pub const KEY_P: i32 = 80;
pub const KEY_Q: i32 = 81;
pub const KEY_R: i32 = 82;
pub const KEY_S: i32 = 83;
pub const KEY_T: i32 = 84;
pub const KEY_U: i32 = 85;
pub const KEY_V: i32 = 86;
pub const KEY_W: i32 = 87;
pub const KEY_X: i32 = 88;
pub const KEY_Y: i32 = 89;
pub const KEY_Z: i32 = 90;

// Function key
pub const KEY_SPACE: i32 = 32;
pub const KEY_ESCAPE: i32 = 256;
pub const KEY_ENTER: i32 = 257;
pub const KEY_TAB: i32 = 258;
pub const KEY_BACKSPACE: i32 = 259;
pub const KEY_INSERT: i32 = 260;
pub const KEY_DELETE: i32 = 261;
pub const KEY_RIGHT: i32 = 262;
pub const KEY_LEFT: i32 = 263;
pub const KEY_DOWN: i32 = 264;
pub const KEY_UP: i32 = 265;
pub const KEY_PAGE_UP: i32 = 266;
pub const KEY_PAGE_DOWN: i32 = 267;
pub const KEY_HOME: i32 = 268;
pub const KEY_END: i32 = 269;
pub const KEY_CAPS_LOCK: i32 = 280;
pub const KEY_SCROLL_LOCK: i32 = 281;
pub const KEY_NUM_LOCK: i32 = 282;
pub const KEY_PRINT_SCREEN: i32 = 283;
pub const KEY_PAUSE: i32 = 284;
pub const KEY_F1: i32 = 290;
pub const KEY_F2: i32 = 291;
pub const KEY_F3: i32 = 292;
pub const KEY_F4: i32 = 293;
pub const KEY_F5: i32 = 294;
pub const KEY_F6: i32 = 295;
pub const KEY_F7: i32 = 296;
pub const KEY_F8: i32 = 297;
pub const KEY_F9: i32 = 298;
pub const KEY_F10: i32 = 299;
pub const KEY_F11: i32 = 300;
pub const KEY_F12: i32 = 301;
pub const KEY_LEFT_SHIFT: i32 = 340;
pub const KEY_LEFT_CONTROL: i32 = 341;
pub const KEY_LEFT_ALT: i32 = 342;
pub const KEY_LEFT_SUPER: i32 = 343;
pub const KEY_RIGHT_SHIFT: i32 = 344;
pub const KEY_RIGHT_CONTROL: i32 = 345;
pub const KEY_RIGHT_ALT: i32 = 346;
pub const KEY_RIGHT_SUPER: i32 = 347;
pub const KEY_KB_MENU: i32 = 348;
pub const KEY_LEFT_BRACKET: i32 = 91;
pub const KEY_BACKSLASH: i32 = 92;
pub const KEY_RIGHT_BRACKET: i32 = 93;
pub const KEY_GRAVE: i32 = 96;

// Keypad key
pub const KEY_KP_0: i32 = 320;
pub const KEY_KP_1: i32 = 321;
pub const KEY_KP_2: i32 = 322;
pub const KEY_KP_3: i32 = 323;
pub const KEY_KP_4: i32 = 324;
pub const KEY_KP_5: i32 = 325;
pub const KEY_KP_6: i32 = 326;
pub const KEY_KP_7: i32 = 327;
pub const KEY_KP_8: i32 = 328;
pub const KEY_KP_9: i32 = 329;
pub const KEY_KP_DECIMAL: i32 = 330;
pub const KEY_KP_DIVIDE: i32 = 331;
pub const KEY_KP_MULTIPLY: i32 = 332;
pub const KEY_KP_SUBTRACT: i32 = 333;
pub const KEY_KP_ADD: i32 = 334;
pub const KEY_KP_ENTER: i32 = 335;
pub const KEY_KP_EQUAL: i32 = 33;

// Structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Color {
    pub r: c_uchar,
    pub g: c_uchar,
    pub b: c_uchar,
    pub a: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vector2 {
    pub x: c_float,
    pub y: c_float,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vector3 {
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
}

// Functions
extern "C" {
    // Core
    // Window-related functions
    pub fn InitWindow(width: c_int, height: c_int, title: *const c_char);
    pub fn WindowShouldClose() -> bool;
    pub fn CloseWindow();
    pub fn IsWindowReady() -> bool;
    pub fn IsWindowMinimized() -> bool;
    pub fn IsWindowResized() -> bool;
    pub fn IsWindowHidden() -> bool;
    pub fn IsWindowFullscreen() -> bool;
    pub fn ToggleFullscreen();
    pub fn UnhideWindow();
    pub fn HideWindow();
    pub fn SetWindowTitle(title: *const c_char);
    pub fn SetWindowPosition(x: c_int, y: c_int);
    pub fn SetWindowMonitor(monitor: c_int);
    pub fn SetWindowMinSize(width: c_int, height: c_int);
    pub fn SetWindowSize(width: c_int, height: c_int);
    pub fn GetScreenWidth() -> c_int;
    pub fn GetScreenHeight() -> c_int;
    pub fn GetMonitorCount() -> c_int;
    pub fn GetMonitorWidth(monitor: c_int) -> c_int;
    pub fn GetMonitorHeight(monitor: c_int) -> c_int;
    pub fn GetMonitorPhysicalWidth(monitor: c_int) -> c_int;
    pub fn GetMonitorPhysicalHeight(monitor: c_int) -> c_int;
    pub fn GetWindowPosition() -> Vector2;
    pub fn GetMonitorName(monitor: c_int) -> *const c_char;
    pub fn GetClipboardText() -> *const c_char;
    pub fn SetClipboardText(text: *const c_char);

    // Drawing-related functions
    pub fn ClearBackground(color: Color);
    pub fn BeginDrawing();
    pub fn EndDrawing();

    // Timing-related functions
    pub fn SetTargetFPS(fps: c_int);

    // Input-related functions: keyb
    pub fn IsKeyDown(key: c_int) -> bool;

    // Shapes
    // Basic shapes drawing functions
    pub fn DrawCircleV(center: Vector2, radius: f32, color: Color);

    // Text
    // Text drawing functions
    pub fn DrawText(text: *const c_char, posX: c_int, posY: c_int, fontSize: c_int, color: Color);
}
