use rayquaza::core::{color::Color, input::Key, math::Vector2, result::Result, window::Window};

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 450;
const BALL_SPEED: f32 = 120.0;

fn main() -> Result {
    let mut window = Window::create(SCREEN_WIDTH, SCREEN_HEIGHT, "Keyboard input")?;
    window.set_target_fps(60);
    let mut ball_position = Vector2::new((SCREEN_WIDTH / 2) as f32, (SCREEN_HEIGHT / 2) as f32);
    while !window.should_close() {
        // Update
        let delta = window.get_frame_time();
        if window.is_key_down(Key::Right) {
            ball_position.x += BALL_SPEED * delta;
        }
        if window.is_key_down(Key::Left) {
            ball_position.x -= BALL_SPEED * delta;
        }
        if window.is_key_down(Key::Up) {
            ball_position.y -= BALL_SPEED * delta;
        }
        if window.is_key_down(Key::Down) {
            ball_position.y += BALL_SPEED * delta;
        }
        // Draw
        window.draw(|canvas| {
            canvas.clear_background(Color::RAYWHITE);
            canvas.draw_circle_vec(ball_position, 50.0, Color::MAROON);
            canvas.draw_text("Move the ball with arrow keys", 10, 10, 20, Color::DARKGRAY);
        });
    }
    Ok(())
}
