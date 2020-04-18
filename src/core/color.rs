use crate::ffi;
use std::mem;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

impl Color {
    pub const LIGHTGRAY: Color = Color {
        red: 200,
        green: 200,
        blue: 200,
        alpha: 255,
    };

    pub const RAYWHITE: Color = Color {
        red: 245,
        green: 245,
        blue: 245,
        alpha: 255,
    };
}

impl From<Color> for ffi::Color {
    fn from(color: Color) -> ffi::Color {
        unsafe { mem::transmute(color) }
    }
}
