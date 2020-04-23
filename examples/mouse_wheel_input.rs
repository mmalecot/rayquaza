use rayquaza::{Color, Result, WindowBuilder};

const BOX_SPEED: f32 = 480.0;
const BOX_SIZE: i32 = 80;

fn main() -> Result {
    let window = WindowBuilder::new()
        .title("Mouse wheel input")
        .vsync()
        .build()?;
    let mut position = (
        window.get_width() / 2 - BOX_SIZE / 2,
        window.get_height() / 2 - BOX_SIZE / 2,
    );
    while !window.should_close() {
        // Updates
        let delta = window.get_frame_time();
        position.1 -= (window.get_mouse_wheel_move() as f32 * BOX_SPEED * delta) as i32;
        // Draws
        window.draw(|canvas| {
            canvas.clear_background(Color::RAYWHITE);
            canvas.draw_rectangle(position.0, position.1, BOX_SIZE, BOX_SIZE, Color::MAROON);
            canvas.draw_text(
                "Use mouse wheel to move the cube up and down!",
                10,
                10,
                20,
                Color::GRAY,
            );
            canvas.draw_text(
                &format!("Box position Y: {:03}", position.1),
                10,
                40,
                20,
                Color::LIGHTGRAY,
            );
        });
    }
    Ok(())
}
