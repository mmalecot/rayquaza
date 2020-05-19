use rayquaza::{
    color::Color, input::MouseButton, math::Vector2, result::Result, window::WindowBuilder,
};

fn main() -> Result {
    let window = WindowBuilder::new()
        .title("Bezier line")
        .resizable()
        .vsync()
        .msaa_4x()
        .build()?;
    let mut start = Vector2::new(100.0, 100.0);
    let mut end = Vector2::new(
        window.width() as f32 - 100.0,
        window.height() as f32 - 100.0,
    );
    while !window.should_close() {
        if window.is_mouse_button_down(MouseButton::Left) {
            start = window.mouse_position();
        }
        if window.is_mouse_button_down(MouseButton::Right) {
            end = window.mouse_position();
        }
        window.draw(|canvas| {
            canvas.clear_background(Color::RAYWHITE);
            canvas.draw_text(
                "Use mouse buttons to define line start and end points",
                20,
                20,
                20,
                Color::GRAY,
            );
            canvas.draw_line_bezier(start, end, 2.0, Color::RED);
        });
    }
    Ok(())
}
