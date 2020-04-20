use rayquaza::core::{color::Color, result::Result, window::Window};

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 450;
const BOX_SPEED: f32 = 240.0;

fn main() -> Result {
    let mut window = Window::create(SCREEN_WIDTH, SCREEN_HEIGHT, "Mouse wheel input")?;
    window.set_target_fps(60);
    let mut box_position = ((SCREEN_WIDTH / 2) - 40, (SCREEN_HEIGHT / 2) - 40);
    while !window.should_close() {
        // Update
        let delta = window.get_frame_time();
        box_position.1 -= (window.get_mouse_wheel_move() as f32 * BOX_SPEED * delta) as i32;
        // Draw
        window.draw(|canvas| {
            canvas.clear_background(Color::RAYWHITE);
            canvas.draw_rectangle(box_position.0, box_position.1, 80, 80, Color::MAROON);
            canvas.draw_text(
                "Use mouse wheel to move the cube up and down!",
                10,
                10,
                20,
                Color::GRAY,
            );
            canvas.draw_text(
                &format!("Box position Y: {:03}", box_position.1),
                10,
                40,
                20,
                Color::LIGHTGRAY,
            );
        });
    }
    Ok(())
}
