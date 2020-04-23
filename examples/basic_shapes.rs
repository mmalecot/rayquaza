use rayquaza::{Color, Result, WindowBuilder};

fn main() -> Result {
    let window = WindowBuilder::new()
        .title("Basic shapes")
        .vsync()
        .msaa_4x()
        .build()?;
    while !window.should_close() {
        window.draw(|canvas| {
            canvas.clear_background(Color::RAYWHITE);
            canvas.draw_text("Some basic shapes available", 20, 20, 20, Color::DARKGRAY);
            canvas.draw_line(18, 42, window.get_screen_width() - 18, 42, Color::BLACK);
            canvas.draw_circle(window.get_screen_width() / 4, 120, 35.0, Color::DARKBLUE);
            canvas.draw_circle_gradient(
                window.get_screen_width() / 4,
                220,
                60.0,
                Color::GREEN,
                Color::SKYBLUE,
            );
            canvas.draw_circle_lines(window.get_screen_width() / 4, 340, 80.0, Color::DARKBLUE);
            canvas.draw_rectangle(
                window.get_screen_width() / 4 * 2 - 60,
                100,
                120,
                60,
                Color::RED,
            );
            canvas.draw_rectangle_horizontal_gradient(
                window.get_screen_width() / 4 * 2 - 90,
                170,
                180,
                130,
                Color::MAROON,
                Color::GOLD,
            );
            canvas.draw_rectangle_lines(
                window.get_screen_width() / 4 * 2 - 40,
                320,
                80,
                60,
                Color::ORANGE,
            );
            canvas.draw_triangle(
                (window.get_screen_width() as f32 / 4.0 * 3.0, 80.0),
                (window.get_screen_width() as f32 / 4.0 * 3.0 - 60.0, 150.0),
                (window.get_screen_width() as f32 / 4.0 * 3.0 + 60.0, 150.0),
                Color::VIOLET,
            );
            canvas.draw_triangle_lines(
                (window.get_screen_width() as f32 / 4.0 * 3.0, 160.0),
                (window.get_screen_width() as f32 / 4.0 * 3.0 - 20.0, 230.0),
                (window.get_screen_width() as f32 / 4.0 * 3.0 + 20.0, 230.0),
                Color::DARKBLUE,
            );
            canvas.draw_polygon_vec(
                (window.get_screen_width() as f32 / 4.0 * 3.0, 320.0),
                6,
                80.0,
                0.0,
                Color::BROWN,
            );
        });
    }
    Ok(())
}
