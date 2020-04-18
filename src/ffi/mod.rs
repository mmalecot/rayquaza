//! raylib raw FFI bindings.

use std::os::raw::{c_char, c_int};

// Structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

// Functions
extern "C" {
    pub fn BeginDrawing();
    pub fn ClearBackground(color: Color);
    pub fn CloseWindow();
    pub fn DrawText(text: *const c_char, posX: c_int, posY: c_int, fontSize: c_int, color: Color);
    pub fn EndDrawing();
    pub fn InitWindow(width: c_int, height: c_int, title: *const c_char);
    pub fn IsWindowReady() -> bool;
    pub fn SetTargetFPS(fps: c_int);
    pub fn WindowShouldClose() -> bool;
}
