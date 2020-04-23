use rayquaza::{Color, Gamepad, Result, WindowBuilder};

fn main() -> Result {
    let window = WindowBuilder::new()
        .size(800, 450)
        .title("Gamepad input")
        .vsync()
        .msaa_4x()
        .build()?;
    while !window.should_close() {
        window.draw(|canvas| {
            canvas.clear_background(Color::RAYWHITE);
            if window.is_gamepad_available(Gamepad::One) {
                canvas.draw_text(
                    &format!("GP1: {}", window.get_gamepad_name(Gamepad::One)),
                    10,
                    10,
                    20,
                    Color::BLACK,
                );
            } else {
                canvas.draw_text("GP1: None", 10, 10, 20, Color::GRAY);
            }
            if let Some(button) = window.get_gamepad_button_pressed() {
                canvas.draw_text(
                    &format!("Detected button: {:?}", button),
                    10,
                    420,
                    20,
                    Color::RED,
                )
            } else {
                canvas.draw_text("Detected button: None", 10, 420, 20, Color::GRAY)
            }
        });
    }
    Ok(())
}
