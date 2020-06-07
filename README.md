# Rayquaza

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/mmalecot/rayquaza/CI)
![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.44+-blueviolet.svg?logo=rust)

![Logo](resources/image/logo.png)

Idiomatic wrapper for [raylib](https://www.raylib.com/).

## Features
- No native dependencies: `raylib` is embedded within the program.
- No Cargo dependencies: build dependencies only.
- Multi-platform: `Windows`, `Linux` and `macOS` (tested via CI for the 3 platforms).
- Idiomatic: `Rust` concepts overuse, safe API, follows `Rust` API & style guidelines.

## Examples

### Basic window

```rust
use rayquaza::{color::Color, result::Result, window::WindowBuilder};

fn main() -> Result {
    let window = WindowBuilder::new()
        .title("Basic window")
        .resizable()
        .vsync()
        .build()?;
    let text = "Congrats! You created your first window!";
    let size = 20;
    let width = window.measure_text(text, size);
    while !window.should_close() {
        window.draw(|canvas| {
            canvas.clear_background(Color::BLACK);
            canvas.draw_text(
                text,
                window.width() / 2 - width / 2,
                window.height() / 2 - size / 2,
                size,
                Color::WHITE,
            );
        });
    }
    Ok(())
}
```

## References

* [Learn Rust](https://www.rust-lang.org/learn)
* [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
* [raylib website](https://www.raylib.com/)

## License

This project is licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
