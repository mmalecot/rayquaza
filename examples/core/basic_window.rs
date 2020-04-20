use rayquaza::core::{color::Color, result::Result, window::Window};

fn main() -> Result {
    let mut window = Window::create(800, 450, "Basic window")?;
    window.set_target_fps(60);
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
