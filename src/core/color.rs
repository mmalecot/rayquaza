//! Color module.

use crate::{core::math, ffi};
use std::{mem, num::ParseIntError, str::FromStr};

/// Convenient macro to create a color.
macro_rules! color {
    ($red:expr, $green:expr, $blue:expr, $alpha:expr) => {
        Color::new($red, $green, $blue, $alpha)
    };
}

/// RGBA color (32-bit).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Color {
    pub const LIGHTGRAY: Color = color!(200, 200, 200, 255);
    pub const GRAY: Color = color!(130, 130, 130, 255);
    pub const DARKGRAY: Color = color!(80, 80, 80, 255);
    pub const YELLOW: Color = color!(253, 249, 0, 255);
    pub const GOLD: Color = color!(255, 203, 0, 255);
    pub const ORANGE: Color = color!(255, 161, 0, 255);
    pub const PINK: Color = color!(255, 109, 194, 255);
    pub const RED: Color = color!(230, 41, 55, 255);
    pub const MAROON: Color = color!(190, 33, 55, 255);
    pub const GREEN: Color = color!(0, 228, 48, 255);
    pub const LIME: Color = color!(0, 158, 47, 255);
    pub const DARKGREEN: Color = color!(0, 117, 44, 255);
    pub const SKYBLUE: Color = color!(102, 191, 255, 255);
    pub const BLUE: Color = color!(0, 121, 241, 255);
    pub const DARKBLUE: Color = color!(0, 82, 172, 255);
    pub const PURPLE: Color = color!(200, 122, 255, 255);
    pub const VIOLET: Color = color!(135, 60, 190, 255);
    pub const DARKPURPLE: Color = color!(112, 31, 126, 255);
    pub const BEIGE: Color = color!(211, 176, 131, 255);
    pub const BROWN: Color = color!(127, 106, 79, 255);
    pub const DARKBROWN: Color = color!(76, 63, 47, 255);
    pub const WHITE: Color = color!(255, 255, 255, 255);
    pub const BLACK: Color = color!(0, 0, 0, 255);
    pub const BLANK: Color = color!(0, 0, 0, 0);
    pub const MAGENTA: Color = color!(255, 0, 255, 255);
    pub const RAYWHITE: Color = color!(245, 245, 245, 255);

    /// Creates a `Color`.
    #[inline]
    pub const fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
        Color {
            red,
            green,
            blue,
            alpha,
        }
    }

    /// Color fade-in or fade-out (alpha goes from 0.0 to 1.0).
    pub fn fade(self, alpha: f32) -> Color {
        color!(
            self.red,
            self.green,
            self.blue,
            ((u8::max_value() as f32) * math::clamp(alpha, 0.0, 1.0)) as u8
        )
    }
}

impl From<&Color> for Color {
    #[inline]
    fn from(color: &Color) -> Color {
        *color
    }
}

impl From<[u8; 3]> for Color {
    #[inline]
    fn from(value: [u8; 3]) -> Color {
        color!(value[0], value[1], value[2], u8::max_value())
    }
}

impl From<[u8; 4]> for Color {
    #[inline]
    fn from(value: [u8; 4]) -> Color {
        color!(value[0], value[1], value[2], value[3])
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    #[inline]
    fn from((red, green, blue, alpha): (u8, u8, u8, u8)) -> Color {
        color!(red, green, blue, alpha)
    }
}

impl From<(u8, u8, u8)> for Color {
    #[inline]
    fn from((red, green, blue): (u8, u8, u8)) -> Color {
        color!(red, green, blue, u8::max_value())
    }
}

impl From<i32> for Color {
    fn from(value: i32) -> Color {
        color!(
            ((value >> 16) & 0xff) as u8,
            ((value >> 8) & 0xff) as u8,
            (value & 0xff) as u8,
            u8::max_value()
        )
    }
}

impl From<u32> for Color {
    fn from(value: u32) -> Color {
        color!(
            ((value >> 24) & 0xff) as u8,
            ((value >> 16) & 0xff) as u8,
            ((value >> 8) & 0xff) as u8,
            (value & 0xff) as u8
        )
    }
}

impl From<ffi::Color> for Color {
    fn from(color: ffi::Color) -> Color {
        unsafe { mem::transmute(color) }
    }
}

impl FromStr for Color {
    type Err = ParseIntError;
    fn from_str(value: &str) -> Result<Color, Self::Err> {
        let value = if let Some(index) = value.find("0x") {
            &value[index + 2..]
        } else {
            value
        };
        Ok(if value.len() == 6 {
            i32::from_str_radix(value, 16)?.into()
        } else {
            u32::from_str_radix(value, 16)?.into()
        })
    }
}

impl Into<[u8; 3]> for Color {
    #[inline]
    fn into(self) -> [u8; 3] {
        [self.red, self.green, self.blue]
    }
}

impl Into<[u8; 4]> for Color {
    #[inline]
    fn into(self) -> [u8; 4] {
        [self.red, self.green, self.blue, self.alpha]
    }
}

impl Into<(u8, u8, u8, u8)> for Color {
    #[inline]
    fn into(self) -> (u8, u8, u8, u8) {
        (self.red, self.green, self.blue, self.alpha)
    }
}

impl Into<(u8, u8, u8)> for Color {
    #[inline]
    fn into(self) -> (u8, u8, u8) {
        (self.red, self.green, self.blue)
    }
}

impl Into<i32> for Color {
    fn into(self) -> i32 {
        ((self.red as i32) << 16) | ((self.green as i32) << 8) | self.blue as i32
    }
}

impl Into<u32> for Color {
    fn into(self) -> u32 {
        ((self.red as u32) << 24)
            | ((self.green as u32) << 16)
            | ((self.blue as u32) << 8)
            | self.alpha as u32
    }
}

impl Into<ffi::Color> for Color {
    fn into(self) -> ffi::Color {
        unsafe { mem::transmute(self) }
    }
}

impl Into<ffi::Color> for &Color {
    fn into(self) -> ffi::Color {
        self.clone().into()
    }
}

impl ToString for Color {
    #[inline]
    fn to_string(&self) -> String {
        format!(
            "0x{:02X}{:02X}{:02X}{:02X}",
            self.red, self.green, self.blue, self.alpha
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::core::color::Color;
    use std::str::FromStr;

    #[test]
    fn test_color_new() {
        assert_eq!(Color::new(0, 121, 241, 255), Color::BLUE);
    }

    #[test]
    fn test_color_fade() {
        assert_eq!(Color::BLUE.fade(1.0), Color::new(0, 121, 241, 255));
        assert_eq!(Color::BLUE.fade(0.5), Color::new(0, 121, 241, 127));
        assert_eq!(Color::BLUE.fade(0.0), Color::new(0, 121, 241, 0));
    }

    #[test]
    fn test_color_from_ref() {
        assert_eq!(Color::from(&Color::BLUE), Color::BLUE);
    }

    #[test]
    fn test_color_from_u8_array() {
        assert_eq!(Color::from([0, 121, 241]), Color::BLUE);
        assert_eq!(Color::from([0, 121, 241, 255]), Color::BLUE);
    }

    #[test]
    fn test_color_from_u8_slice() {
        assert_eq!(Color::from((0, 121, 241, 255)), Color::BLUE);
        assert_eq!(Color::from((0, 121, 241)), Color::BLUE);
    }

    #[test]
    fn test_color_from_i32() {
        assert_eq!(Color::from(0x00_79_F1), Color::BLUE);
    }

    #[test]
    fn test_color_from_u32() {
        assert_eq!(Color::from(0x00_79_F1_FFu32), Color::BLUE);
    }

    #[test]
    fn test_color_from_str() {
        assert_eq!(Color::from_str("0x0079F1").unwrap(), Color::BLUE);
        assert_eq!(Color::from_str("0x0079F1FF").unwrap(), Color::BLUE);
        assert_eq!(Color::from_str("0079F1").unwrap(), Color::BLUE);
        assert_eq!(Color::from_str("0079F1FF").unwrap(), Color::BLUE);
    }

    #[test]
    fn test_color_into_u8_array() {
        let value: [u8; 3] = Color::BLUE.into();
        assert_eq!(value, [0, 121, 241]);
        let value: [u8; 4] = Color::BLUE.into();
        assert_eq!(value, [0, 121, 241, 255]);
    }

    #[test]
    fn test_color_into_u8_slice() {
        let value: (u8, u8, u8, u8) = Color::BLUE.into();
        assert_eq!(value, (0, 121, 241, 255));
        let value: (u8, u8, u8) = Color::BLUE.into();
        assert_eq!(value, (0, 121, 241));
    }

    #[test]
    fn test_color_into_i32() {
        let value: i32 = Color::BLUE.into();
        assert_eq!(value, 0x00_79_F1);
    }

    #[test]
    fn test_color_into_u32() {
        let value: u32 = Color::BLUE.into();
        assert_eq!(value, 0x00_79_F1_FF);
    }

    #[test]
    fn test_color_to_string() {
        assert_eq!(Color::BLUE.to_string(), String::from("0x0079F1FF"));
    }
}
