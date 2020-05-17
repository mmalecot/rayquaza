//! Miscellaneous items.

use crate::ffi;

/// Returns a random value between `min` and `max` (both included).
pub fn get_random_value(min: i32, max: i32) -> i32 {
    unsafe { ffi::GetRandomValue(min, max) }
}
