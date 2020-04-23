//! Idiomatic wrapper for [raylib](https://www.raylib.com/).
//!
//! # Features
//!
//! - No native dependencies: `raylib` is embedded within the program.
//! - No Cargo dependencies: use only build dependencies.
//! - Multi-platform: `Windows`, `Linux` and `macOS` (tested via CI for the 3 platforms).
//! - Idiomatic: `Rust` concepts overuse, safe API, follows `Rust` API & style guidelines.
//!
//! # Example
//!
//! ```rust,no_run
//! use rayquaza::{Color, Result, WindowBuilder};
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
