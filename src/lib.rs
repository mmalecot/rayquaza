//! Idiomatic wrapper for [raylib](https://www.raylib.com/).
//!
//! # Hello world
//!
//! ```rust,no_run
//! use rayquaza::core::{color::Color, result::Result, window::WindowBuilder};
//!
//! fn main() -> Result {
//!     let mut window = WindowBuilder::new()
//!         .size(800, 450)
//!         .title("Basic window")
//!         .vsync()
//!         .build()?;
//!     while !window.should_close() {
//!         window.draw(|canvas| {
//!             canvas.clear_background(Color::WHITE);
//!             canvas.draw_text("Hello world!", 268, 200, 48, Color::BLACK);
//!         });
//!     }
//!     Ok(())
//! }
//! ```

// Internal modules
pub(crate) mod ffi;

// Public modules
pub mod core;
