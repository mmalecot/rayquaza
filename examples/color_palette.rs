use rayquaza::{
    collision::check_point_rectangle, color::Color, input::Key, math::Rectangle, result::Result,
    text::measure_text, window::WindowBuilder,
};

#[derive(PartialEq)]
enum State {
    Normal,
    Hover,
}

fn main() -> Result {
    let window = WindowBuilder::new()
        .title("Color palette")
        .vsync()
        .build()?;
    let mut colors = vec![
        (Color::DARKGRAY, "DARKGRAY", State::Normal),
        (Color::MAROON, "MAROON", State::Normal),
        (Color::ORANGE, "ORANGE", State::Normal),
        (Color::DARKGREEN, "DARKGREEN", State::Normal),
        (Color::DARKBLUE, "DARKBLUE", State::Normal),
        (Color::DARKPURPLE, "DARKPURPLE", State::Normal),
        (Color::DARKBROWN, "DARKBROWN", State::Normal),
        (Color::GRAY, "GRAY", State::Normal),
        (Color::RED, "RED", State::Normal),
        (Color::GOLD, "GOLD", State::Normal),
        (Color::LIME, "LIME", State::Normal),
        (Color::BLUE, "BLUE", State::Normal),
        (Color::VIOLET, "VIOLET", State::Normal),
        (Color::BROWN, "BROWN", State::Normal),
        (Color::LIGHTGRAY, "LIGHTGRAY", State::Normal),
        (Color::PINK, "PINK", State::Normal),
        (Color::YELLOW, "YELLOW", State::Normal),
        (Color::GREEN, "GREEN", State::Normal),
        (Color::SKYBLUE, "SKYBLUE", State::Normal),
        (Color::PURPLE, "PURPLE", State::Normal),
        (Color::BEIGE, "BEIGE", State::Normal),
    ];
    let rectangles: Vec<_> = colors
        .iter()
        .enumerate()
        .map(|(index, _)| {
            Rectangle::new(
                20.0 + 100.0 * (index % 7) as f32 + 10.0 * (index % 7) as f32,
                80.0 + 100.0 * (index / 7) as f32 + 10.0 * (index / 7) as f32,
                100.0,
                100.0,
            )
        })
        .collect();
    while !window.should_close() {
        // Updates
        for index in 0..colors.len() {
            colors[index].2 =
                if check_point_rectangle(window.get_mouse_position(), rectangles[index]) {
                    State::Hover
                } else {
                    State::Normal
                }
        }
        // Draws
        window.draw(|canvas| {
            canvas.clear_background(Color::RAYWHITE);
            canvas.draw_text("Color palette", 20, 30, 20, Color::BLACK);
            for index in 0..colors.len() {
                canvas.draw_rectangle_rec(
                    rectangles[index],
                    colors[index].0.fade(if colors[index].2 == State::Hover {
                        0.6
                    } else {
                        1.0
                    }),
                );
                if window.is_key_down(Key::Space) || colors[index].2 == State::Hover {
                    canvas.draw_rectangle(
                        rectangles[index].x as i32,
                        rectangles[index].y as i32 + rectangles[index].height as i32 - 26,
                        rectangles[index].width as i32,
                        20,
                        Color::BLACK,
                    );
                    canvas.draw_rectangle_lines_ex(rectangles[index], 6, Color::BLACK.fade(0.3));
                    canvas.draw_text(
                        colors[index].1,
                        rectangles[index].x as i32 + rectangles[index].width as i32
                            - measure_text(colors[index].1, 10)
                            - 12,
                        rectangles[index].y as i32 + rectangles[index].height as i32 - 20,
                        10,
                        colors[index].0,
                    );
                }
            }
            canvas.draw_text(
                "Press SPACE to see all colors",
                window.get_width() - 180,
                window.get_height() - 40,
                10,
                Color::GRAY,
            );
        });
    }
    Ok(())
}
