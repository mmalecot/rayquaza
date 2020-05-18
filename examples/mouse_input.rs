use rayquaza::{
    color::Color, input::MouseButton, math::Vector2, result::Result, window::WindowBuilder,
};

struct Ball {
    position: Vector2,
    color: Color,
}

fn main() -> Result {
    let window = WindowBuilder::new()
        .title("Mouse input")
        .resizable()
        .vsync()
        .msaa_4x()
        .build()?;
    let mut ball = Ball {
        position: Vector2::default(),
        color: Color::DARKBLUE,
    };
    while !window.should_close() {
        ball.position = window.get_mouse_position();
        ball.color = if window.is_mouse_button_pressed(MouseButton::Left) {
            Color::MAROON
        } else if window.is_mouse_button_pressed(MouseButton::Right) {
            Color::DARKBLUE
        } else if window.is_mouse_button_pressed(MouseButton::Middle) {
            Color::LIME
        } else {
            ball.color
        };
        window.draw(|canvas| {
            canvas.clear_background(Color::RAYWHITE);
            canvas.draw_circle_vec(ball.position, 40.0, ball.color);
            canvas.draw_text(
                "Move ball with mouse and click mouse buttons to change color",
                10,
                10,
                20,
                Color::DARKGRAY,
            );
        });
    }
    Ok(())
}
