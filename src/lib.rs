//! Idiomatic wrapper for [raylib](https://www.raylib.com/).
//!
//! # Hello world
//!
//! ```rust,no_run
//! use rayquaza::{Color, Result, WindowBuilder};
//!
//! fn main() -> Result {
//!     let window = WindowBuilder::new()
//!         .size(800, 450)
//!         .title("Basic window")
//!         .vsync()
//!         .build()?;
//!     while !window.should_close() {
//!         window.draw(|canvas| {
//!             canvas.clear_background(Color::BLACK);
//!             canvas.draw_text("Hello world!", 268, 200, 48, Color::WHITE);
//!         });
//!     }
//!     Ok(())
//! }
//! ```

// Private modules
mod ffi;

// Internal modules
mod color;
mod drawing;
mod error;
mod input;
mod math;
mod result;
mod shapes;
mod text;
mod texture;
mod timing;
mod window;

// Re-exported items
pub use crate::{
    color::*, drawing::*, error::*, input::*, math::*, result::*, shapes::*, text::*, texture::*,
    timing::*, window::*,
};
