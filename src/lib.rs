//! Idiomatic wrapper for [raylib](https://www.raylib.com/).
//!
//! # Features
//!
//! - No native dependencies: `raylib` is embedded within the program.
//! - No Cargo dependencies: build dependencies only.
//! - Multi-platform: `Windows`, `Linux` and `macOS` (tested via CI for the 3 platforms).
//! - Idiomatic: `Rust` concepts overuse, safe API, follows `Rust` API & style guidelines.
//!
//! # Example
//!
//! ```rust,no_run
//! use rayquaza::{color::Color, result::Result, window::WindowBuilder};
//!
//! fn main() -> Result {
//!     let window = WindowBuilder::new().build()?;
//!     while !window.should_close() {
//!         window.draw(|canvas| {
//!             canvas.clear_background(Color::BLACK);
//!             canvas.draw_text("Hello world!", 268, 200, 48, Color::WHITE);
//!         });
//!     }
//!     Ok(())
//! }
//! ```

// Internal modules
mod cursor;
mod ffi;
mod shapes;
mod timing;

// Public modules
pub mod camera;
pub mod collision;
pub mod color;
pub mod drawing;
pub mod error;
pub mod input;
pub mod math;
pub mod misc;
pub mod result;
pub mod text;
pub mod texture;
pub mod window;
