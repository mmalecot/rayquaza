//! Utilities related to FFI bindings..

use std::os::raw::{c_char, c_double, c_float, c_int, c_uchar, c_uint};

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

// Mouse buttons
pub const MOUSE_LEFT_BUTTON: i32 = 0;
pub const MOUSE_RIGHT_BUTTON: i32 = 1;
pub const MOUSE_MIDDLE_BUTTON: i32 = 2;

// Gamepad axis
pub const GAMEPAD_AXIS_LEFT_X: i32 = 0;
pub const GAMEPAD_AXIS_LEFT_Y: i32 = 1;
pub const GAMEPAD_AXIS_RIGHT_X: i32 = 2;
pub const GAMEPAD_AXIS_RIGHT_Y: i32 = 3;
pub const GAMEPAD_AXIS_LEFT_TRIGGER: i32 = 4;
pub const GAMEPAD_AXIS_RIGHT_TRIGGER: i32 = 5;

// Gamepad number
pub const GAMEPAD_PLAYER1: i32 = 0;
pub const GAMEPAD_PLAYER2: i32 = 1;
pub const GAMEPAD_PLAYER3: i32 = 2;
pub const GAMEPAD_PLAYER4: i32 = 3;

// System config flags
pub const FLAG_FULLSCREEN_MODE: u32 = 2;
pub const FLAG_WINDOW_RESIZABLE: u32 = 4;
pub const FLAG_WINDOW_UNDECORATED: u32 = 8;
pub const FLAG_WINDOW_TRANSPARENT: u32 = 16;
pub const FLAG_WINDOW_HIDDEN: u32 = 128;
pub const FLAG_WINDOW_ALWAYS_RUN: u32 = 256;
pub const FLAG_MSAA_4X_HINT: u32 = 32;
pub const FLAG_VSYNC_HINT: u32 = 64;

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
pub struct Rectangle {
    pub x: c_float,
    pub y: c_float,
    pub width: c_float,
    pub height: c_float,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Texture2D {
    pub id: c_uint,
    pub width: c_int,
    pub height: c_int,
    pub mipmaps: c_int,
    pub format: c_int,
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

    // Cursor-related functions
    pub fn ShowCursor();
    pub fn HideCursor();
    pub fn IsCursorHidden() -> bool;
    pub fn EnableCursor();
    pub fn DisableCursor();

    // Drawing-related functions
    pub fn ClearBackground(color: Color);
    pub fn BeginDrawing();
    pub fn EndDrawing();

    // Timing-related functions
    pub fn SetTargetFPS(fps: c_int);
    pub fn GetFPS() -> c_int;
    pub fn GetFrameTime() -> c_float;
    pub fn GetTime() -> c_double;

    // Misc. functions
    pub fn SetConfigFlags(flags: c_uint);

    // Input-related functions: keyboard
    pub fn IsKeyPressed(key: c_int) -> bool;
    pub fn IsKeyDown(key: c_int) -> bool;
    pub fn IsKeyReleased(key: c_int) -> bool;
    pub fn IsKeyUp(key: c_int) -> bool;
    pub fn GetKeyPressed() -> c_int;
    pub fn SetExitKey(key: c_int);

    // Input-related functions: gamepads
    pub fn IsGamepadAvailable(gamepad: c_int) -> bool;
    pub fn IsGamepadName(gamepad: c_int, name: *const c_char) -> bool;
    pub fn GetGamepadName(gamepad: c_int) -> *const c_char;
    pub fn IsGamepadButtonPressed(gamepad: c_int, button: c_int) -> bool;
    pub fn IsGamepadButtonDown(gamepad: c_int, button: c_int) -> bool;
    pub fn IsGamepadButtonReleased(gamepad: c_int, button: c_int) -> bool;
    pub fn IsGamepadButtonUp(gamepad: c_int, button: c_int) -> bool;
    pub fn GetGamepadButtonPressed() -> c_int;
    pub fn GetGamepadAxisCount(gamepad: c_int) -> c_int;
    pub fn GetGamepadAxisMovement(gamepad: c_int, axis: c_int) -> c_float;

    // Input-related functions: mouse
    pub fn IsMouseButtonPressed(button: c_int) -> bool;
    pub fn IsMouseButtonDown(button: c_int) -> bool;
    pub fn IsMouseButtonReleased(button: c_int) -> bool;
    pub fn IsMouseButtonUp(button: c_int) -> bool;
    pub fn GetMouseX() -> c_int;
    pub fn GetMouseY() -> c_int;
    pub fn GetMousePosition() -> Vector2;
    pub fn SetMousePosition(x: c_int, y: c_int);
    pub fn SetMouseOffset(offsetX: c_int, offsetY: c_int);
    pub fn SetMouseScale(scaleX: c_float, scaleY: c_float);
    pub fn GetMouseWheelMove() -> c_int;

    // Shapes
    // Basic shapes drawing functions
    pub fn DrawPixel(posX: c_int, posY: c_int, color: Color);
    pub fn DrawPixelV(position: Vector2, color: Color);
    pub fn DrawLine(
        startPosX: c_int,
        startPosY: c_int,
        endPosX: c_int,
        endPosY: c_int,
        color: Color,
    );
    pub fn DrawLineV(startPos: Vector2, endPos: Vector2, color: Color);
    pub fn DrawLineEx(startPos: Vector2, endPos: Vector2, thick: c_float, color: Color);
    pub fn DrawLineBezier(startPos: Vector2, endPos: Vector2, thick: c_float, color: Color);
    pub fn DrawLineStrip(points: *mut Vector2, numPoints: c_int, color: Color);
    pub fn DrawCircle(centerX: c_int, centerY: c_int, radius: f32, color: Color);
    pub fn DrawCircleSector(
        center: Vector2,
        radius: c_float,
        startAngle: c_int,
        endAngle: c_int,
        segments: c_int,
        color: Color,
    );
    pub fn DrawCircleSectorLines(
        center: Vector2,
        radius: c_float,
        startAngle: c_int,
        endAngle: c_int,
        segments: c_int,
        color: Color,
    );
    pub fn DrawCircleGradient(
        centerX: c_int,
        centerY: c_int,
        radius: f32,
        color1: Color,
        color2: Color,
    );
    pub fn DrawCircleV(center: Vector2, radius: f32, color: Color);
    pub fn DrawCircleLines(centerX: c_int, centerY: c_int, radius: f32, color: Color);
    pub fn DrawEllipse(
        centerX: c_int,
        centerY: c_int,
        radiusH: c_float,
        radiusV: c_float,
        color: Color,
    );
    pub fn DrawEllipseLines(
        centerX: c_int,
        centerY: c_int,
        radiusH: c_float,
        radiusV: c_float,
        color: Color,
    );
    pub fn DrawRing(
        center: Vector2,
        innerRadius: c_float,
        outerRadius: c_float,
        startAngle: c_int,
        endAngle: c_int,
        segments: c_int,
        color: Color,
    );
    pub fn DrawRingLines(
        center: Vector2,
        innerRadius: c_float,
        outerRadius: c_float,
        startAngle: c_int,
        endAngle: c_int,
        segments: c_int,
        color: Color,
    );
    pub fn DrawRectangle(posX: c_int, posY: c_int, width: c_int, height: c_int, color: Color);
    pub fn DrawRectangleV(position: Vector2, size: Vector2, color: Color);
    pub fn DrawRectangleRec(rec: Rectangle, color: Color);
    pub fn DrawRectanglePro(rec: Rectangle, origin: Vector2, rotation: c_float, color: Color);
    pub fn DrawRectangleGradientV(
        posX: c_int,
        posY: c_int,
        width: c_int,
        height: c_int,
        color1: Color,
        color2: Color,
    );
    pub fn DrawRectangleGradientH(
        posX: c_int,
        posY: c_int,
        width: c_int,
        height: c_int,
        color1: Color,
        color2: Color,
    );
    pub fn DrawRectangleGradientEx(
        rec: Rectangle,
        col1: Color,
        col2: Color,
        col3: Color,
        col4: Color,
    );
    pub fn DrawRectangleLines(posX: c_int, posY: c_int, width: c_int, height: c_int, color: Color);
    pub fn DrawRectangleLinesEx(rec: Rectangle, lineThick: c_int, color: Color);
    pub fn DrawRectangleRounded(rec: Rectangle, roundness: c_float, segments: c_int, color: Color);
    pub fn DrawRectangleRoundedLines(
        rec: Rectangle,
        roundness: c_float,
        segments: c_int,
        color: Color,
    );
    pub fn DrawTriangle(v1: Vector2, v2: Vector2, v3: Vector2, color: Color);
    pub fn DrawTriangleLines(v1: Vector2, v2: Vector2, v3: Vector2, color: Color);
    pub fn DrawTriangleFan(points: *mut Vector2, numPoints: c_int, color: Color);
    pub fn DrawTriangleStrip(points: *mut Vector2, pointsCount: c_int, color: Color);
    pub fn DrawPoly(
        center: Vector2,
        sides: c_int,
        radius: c_float,
        rotation: c_float,
        color: Color,
    );
    pub fn DrawPolyLines(
        center: Vector2,
        sides: c_int,
        radius: c_float,
        rotation: c_float,
        color: Color,
    );
    // Basic shapes collision detection functions
    pub fn CheckCollisionRecs(rec1: Rectangle, rec2: Rectangle) -> bool;
    pub fn CheckCollisionCircles(
        center1: Vector2,
        radius1: c_float,
        center2: Vector2,
        radius2: c_float,
    ) -> bool;
    pub fn CheckCollisionCircleRec(center: Vector2, radius: c_float, rec: Rectangle) -> bool;
    pub fn GetCollisionRec(rec1: Rectangle, rec2: Rectangle) -> Rectangle;
    pub fn CheckCollisionPointRec(point: Vector2, rec: Rectangle) -> bool;
    pub fn CheckCollisionPointCircle(point: Vector2, center: Vector2, radius: c_float) -> bool;
    pub fn CheckCollisionPointTriangle(
        point: Vector2,
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
    ) -> bool;

    // Textures
    // Texture loading function
    pub fn LoadTexture(fileName: *const c_char) -> Texture2D;
    pub fn UnloadTexture(texture: Texture2D);

    // Texture drawing functions
    pub fn DrawTexture(texture: Texture2D, posX: c_int, posY: c_int, tint: Color);

    // Text
    // Text drawing functions
    pub fn DrawFPS(x: c_int, y: c_int);
    pub fn DrawText(text: *const c_char, posX: c_int, posY: c_int, fontSize: c_int, color: Color);

    // Text misc. functions
    pub fn MeasureText(text: *const c_char, fontSize: c_int) -> c_int;
}
