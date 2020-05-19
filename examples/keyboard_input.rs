use rayquaza::{color::Color, input::Key, math::Vector2, result::Result, window::WindowBuilder};

const BALL_SPEED: f32 = 200.0;

fn main() -> Result {
    let window = WindowBuilder::new()
        .title("Keyboard input")
        .vsync()
        .msaa_4x()
        .build()?;
    let mut position = Vector2::new(window.width() as f32 / 2.0, window.height() as f32 / 2.0);
    while !window.should_close() {
        if window.is_key_down(Key::Right) {
            position.x += BALL_SPEED * window.frame_time();
        }
        if window.is_key_down(Key::Left) {
            position.x -= BALL_SPEED * window.frame_time();
        }
        if window.is_key_down(Key::Up) {
            position.y -= BALL_SPEED * window.frame_time();
        }
        if window.is_key_down(Key::Down) {
            position.y += BALL_SPEED * window.frame_time();
        }
        window.draw(|canvas| {
            canvas.clear_background(Color::RAYWHITE);
            canvas.draw_circle_vec(position, 50.0, Color::MAROON);
            canvas.draw_text("Move the ball with arrow keys", 10, 10, 20, Color::DARKGRAY);
        });
    }
    Ok(())
}
