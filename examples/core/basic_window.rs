use rayquaza::core::{color::Color, result::Result, window::WindowBuilder};

fn main() -> Result {
    let window = WindowBuilder::new()
        .size(800, 450)
        .title("Basic window")
        .vsync()
        .build()?;
    while !window.should_close() {
        window.draw(|canvas| {
            canvas.clear_background(Color::RAYWHITE);
            canvas.draw_text(
                "Congrats! You created your first window!",
                190,
                200,
                20,
                Color::LIGHTGRAY,
            );
        });
    }
    Ok(())
}
