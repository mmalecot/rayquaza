use rayquaza::{
    color::Color,
    input::{Gamepad, GamepadAxis, GamepadButton},
    math::Vector2,
    result::Result,
    window::WindowBuilder,
};

const BALL_SPEED: f32 = 200.0;

fn main() -> Result {
    let window = WindowBuilder::new()
        .title("Gamepad input")
        .vsync()
        .msaa_4x()
        .build()?;
    let axis = vec![
        GamepadAxis::LeftX,
        GamepadAxis::LeftY,
        GamepadAxis::RightX,
        GamepadAxis::RightY,
        GamepadAxis::LeftTrigger,
        GamepadAxis::RightTrigger,
    ];
    let mut position = Vector2::new(
        window.get_width() as f32 * 3.0 / 4.0,
        window.get_height() as f32 / 2.0,
    );
    while !window.should_close() {
        // Updates
        let delta = window.get_frame_time();
        if window.is_gamepad_button_down(Gamepad::One, GamepadButton::LeftFaceRight) {
            position.x += BALL_SPEED * delta;
        }
        if window.is_gamepad_button_down(Gamepad::One, GamepadButton::LeftFaceLeft) {
            position.x -= BALL_SPEED * delta;
        }
        if window.is_gamepad_button_down(Gamepad::One, GamepadButton::LeftFaceUp) {
            position.y -= BALL_SPEED * delta;
        }
        if window.is_gamepad_button_down(Gamepad::One, GamepadButton::LeftFaceDown) {
            position.y += BALL_SPEED * delta;
        }
        // Draws
        window.draw(|canvas| {
            canvas.clear_background(Color::BLACK);
            canvas.draw_circle_vec(position, 50.0, Color::MAROON);
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
                Color::WHITE,
            );
            canvas.draw_text("Axis:", 10, 60, 20, Color::WHITE);
            axis.iter().enumerate().for_each(|(index, axis)| {
                canvas.draw_text(
                    &format!(
                        "{:?}: {:.2}",
                        axis.clone(),
                        window.get_gamepad_axis_movement(Gamepad::One, axis.clone())
                    ),
                    20,
                    100 + (index as i32 * 30),
                    20,
                    Color::WHITE,
                );
            });
            if let Some(button) = window.get_gamepad_button_pressed() {
                canvas.draw_text(
                    &format!("Detected button: {:?}", button),
                    10,
                    420,
                    20,
                    Color::GREEN,
                );
            } else {
                canvas.draw_text("Detected button: None", 10, 420, 20, Color::WHITE);
            }
        });
    }
    Ok(())
}
