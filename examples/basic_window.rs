use rayquaza::core::{color::Color, error::Error, window::Window};

fn main() -> Result<(), Error> {
    let mut window = Window::create(800, 450, "Basic window")?;
    window.set_target_fps(60);
    while !window.should_close() {
        let drawing = window.drawing();
        drawing.clear_background(Color::RAYWHITE);
        drawing.draw_text(
            "Congrats! You created your first window!",
            190,
            200,
            20,
            Color::LIGHTGRAY,
        );
    }
    Ok(())
}
