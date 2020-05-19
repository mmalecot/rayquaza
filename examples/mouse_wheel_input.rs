use rayquaza::{color::Color, math::Vector2, result::Result, window::WindowBuilder};

const BOX_SIZE: i32 = 80;
const BOX_SPEED: f32 = 480.0;

fn main() -> Result {
    let window = WindowBuilder::new()
        .title("Mouse wheel input")
        .vsync()
        .build()?;
    let mut position = Vector2::new(
        window.width() as f32 / 2.0 - BOX_SIZE as f32 / 2.0,
        window.height() as f32 / 2.0 - BOX_SIZE as f32 / 2.0,
    );
    while !window.should_close() {
        position.y -= window.mouse_wheel_move() as f32 * BOX_SPEED * window.frame_time();
        window.draw(|canvas| {
            canvas.clear_background(Color::RAYWHITE);
            canvas.draw_rectangle(
                position.x as i32,
                position.y as i32,
                BOX_SIZE,
                BOX_SIZE,
                Color::MAROON,
            );
            canvas.draw_text(
                "Use mouse wheel to move the cube up and down!",
                10,
                10,
                20,
                Color::GRAY,
            );
            canvas.draw_text(
                &format!("Box position Y: {}", position.y as i32),
                10,
                40,
                20,
                Color::LIGHTGRAY,
            );
        });
    }
    Ok(())
}
