use rayquaza::core::{
    color::Color, input::Key, math::Vector2, result::Result, window::WindowBuilder,
};

const BALL_SPEED: f32 = 120.0;

fn main() -> Result {
    let window = WindowBuilder::new()
        .size(800, 450)
        .title("Keyboard input")
        .vsync()
        .msaa_4x()
        .build()?;
    let mut position = Vector2::new(
        window.get_screen_width() as f32 / 2.0,
        window.get_screen_height() as f32 / 2.0,
    );
    while !window.should_close() {
        // Update
        let delta = window.get_frame_time();
        if window.is_key_down(Key::Right) {
            position.x += BALL_SPEED * delta;
        }
        if window.is_key_down(Key::Left) {
            position.x -= BALL_SPEED * delta;
        }
        if window.is_key_down(Key::Up) {
            position.y -= BALL_SPEED * delta;
        }
        if window.is_key_down(Key::Down) {
            position.y += BALL_SPEED * delta;
        }
        // Draw
        window.draw(|canvas| {
            canvas.clear_background(Color::RAYWHITE);
            canvas.draw_circle_vec(position, 50.0, Color::MAROON);
            canvas.draw_text("Move the ball with arrow keys", 10, 10, 20, Color::DARKGRAY);
        });
    }
    Ok(())
}
