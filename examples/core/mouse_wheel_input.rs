use rayquaza::core::{color::Color, result::Result, window::WindowBuilder};

const BOX_SPEED: f32 = 240.0;
const BOX_SIZE: i32 = 80;

fn main() -> Result {
    let mut window = WindowBuilder::new()
        .size(800, 450)
        .title("Mouse wheel input")
        .vsync()
        .build()?;
    let mut box_position = (
        window.get_screen_width() / 2 - BOX_SIZE / 2,
        window.get_screen_height() / 2 - BOX_SIZE / 2,
    );
    while !window.should_close() {
        // Update
        let delta = window.get_frame_time();
        box_position.1 -= (window.get_mouse_wheel_move() as f32 * BOX_SPEED * delta) as i32;
        // Draw
        window.draw(|canvas| {
            canvas.clear_background(Color::RAYWHITE);
            canvas.draw_rectangle(
                box_position.0,
                box_position.1,
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
