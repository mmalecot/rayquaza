use rayquaza::{color::Color, input::Gamepad, result::Result, window::WindowBuilder};

fn main() -> Result {
    let window = WindowBuilder::new()
        .title("Gamepad input")
        .vsync()
        .msaa_4x()
        .build()?;
    while !window.should_close() {
        window.draw(|canvas| {
            canvas.clear_background(Color::RAYWHITE);
            canvas.draw_text(
                &format!(
                    "GP1: {}",
                    window
                        .get_gamepad_name(Gamepad::One)
                        .unwrap_or_else(|| String::from("None"))
                ),
                10,
                10,
                20,
                Color::BLACK,
            );
            if let Some(button) = window.get_gamepad_button_pressed() {
                canvas.draw_text(
                    &format!("Detected button: {:?}", button),
                    10,
                    420,
                    20,
                    Color::RED,
                );
            } else {
                canvas.draw_text("Detected button: None", 10, 420, 20, Color::GRAY);
            }
        });
    }
    Ok(())
}
