use rayquaza::core::{color::Color, input::Button, math::Vector2, result::Result, window::Window};

fn main() -> Result {
    let mut window = Window::create(800, 450, "Mouse input")?;
    window.set_target_fps(60);
    let mut ball = (Vector2::default(), Color::DARKBLUE);
    while !window.should_close() {
        // Update
        ball.0 = window.get_mouse_position();
        ball.1 = if window.is_mouse_button_pressed(Button::Left) {
            Color::MAROON
        } else if window.is_mouse_button_pressed(Button::Right) {
            Color::DARKBLUE
        } else if window.is_mouse_button_pressed(Button::Middle) {
            Color::LIME
        } else {
            ball.1
        };
        // Draw
        window.draw(|canvas| {
            canvas.clear_background(Color::RAYWHITE);
            canvas.draw_circle_vec(ball.0, 40.0, ball.1);
            canvas.draw_text(
                "Move ball with mouse and click mouse button to change color",
                10,
                10,
                20,
                Color::DARKGRAY,
            );
        });
    }
    Ok(())
}
