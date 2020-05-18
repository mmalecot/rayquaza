use rayquaza::{
    collision::{check_rectangles, get_collision_rectangle},
    color::Color,
    math::{clamp, Rectangle},
    result::Result,
    window::WindowBuilder,
};

fn main() -> Result {
    let window = WindowBuilder::new()
        .title("Collision area")
        .vsync()
        .msaa_4x()
        .build()?;
    let mut rectangle1 = Rectangle::new(0.0, window.get_height() as f32 / 2.0 - 50.0, 200.0, 100.0);
    let mut rectangle2 = Rectangle::new(
        window.get_width() as f32 / 2.0 - 30.0,
        window.get_height() as f32 / 2.0 - 30.0,
        60.0,
        60.0,
    );
    let mut motion = 240.0;
    while !window.should_close() {
        rectangle1.x = clamp(
            rectangle1.x + motion * window.get_frame_time(),
            0.0,
            window.get_width() as f32 - rectangle1.width,
        );
        if rectangle1.x == 0.0 || rectangle1.x == window.get_width() as f32 - rectangle1.width {
            motion *= -1.0;
        }
        rectangle2.x = clamp(
            window.get_mouse_x() as f32 - rectangle2.width / 2.0,
            0.0,
            window.get_width() as f32 - rectangle2.width,
        );
        rectangle2.y = clamp(
            window.get_mouse_y() as f32 - rectangle2.height / 2.0,
            40.0,
            window.get_height() as f32 - rectangle2.height,
        );
        let collision = if check_rectangles(rectangle1, rectangle2) {
            Some(get_collision_rectangle(rectangle1, rectangle2))
        } else {
            None
        };
        window.draw(|canvas| {
            canvas.clear_background(Color::RAYWHITE);
            canvas.draw_rectangle_rec(rectangle1, Color::GOLD);
            canvas.draw_rectangle_rec(rectangle2, Color::BLUE);
            if let Some(rectangle) = collision {
                canvas.draw_rectangle_rec(rectangle, Color::LIME);
            }
            canvas.draw_rectangle(
                0,
                0,
                window.get_width(),
                40,
                if collision.is_some() {
                    Color::RED
                } else {
                    Color::BLACK
                },
            );
            if let Some(rectangle) = collision {
                canvas.draw_text(
                    &format!("Collision area: {}", rectangle.area() as i64),
                    window.get_width() / 2 - 100,
                    10,
                    20,
                    Color::WHITE,
                );
            }
        });
    }
    Ok(())
}
