use rayquaza::{
    collision::check_point_rectangle, color::Color, input::Key, math::Rectangle, result::Result,
    text::measure_text, window::WindowBuilder,
};

#[derive(PartialEq)]
enum State {
    Normal,
    Hover,
}

struct Item {
    color: Color,
    title: String,
    state: State,
    rectangle: Rectangle,
}

fn main() -> Result {
    let window = WindowBuilder::new()
        .title("Color palette")
        .vsync()
        .build()?;
    let mut items = vec![
        Item {
            color: Color::DARKGRAY,
            title: String::from("DARKGRAY"),
            state: State::Normal,
            rectangle: Rectangle::default(),
        },
        Item {
            color: Color::MAROON,
            title: String::from("MAROON"),
            state: State::Normal,
            rectangle: Rectangle::default(),
        },
        Item {
            color: Color::ORANGE,
            title: String::from("ORANGE"),
            state: State::Normal,
            rectangle: Rectangle::default(),
        },
        Item {
            color: Color::DARKGREEN,
            title: String::from("DARKGREEN"),
            state: State::Normal,
            rectangle: Rectangle::default(),
        },
        Item {
            color: Color::DARKBLUE,
            title: String::from("DARKBLUE"),
            state: State::Normal,
            rectangle: Rectangle::default(),
        },
        Item {
            color: Color::DARKPURPLE,
            title: String::from("DARKPURPLE"),
            state: State::Normal,
            rectangle: Rectangle::default(),
        },
        Item {
            color: Color::DARKBROWN,
            title: String::from("DARKBROWN"),
            state: State::Normal,
            rectangle: Rectangle::default(),
        },
        Item {
            color: Color::GRAY,
            title: String::from("GRAY"),
            state: State::Normal,
            rectangle: Rectangle::default(),
        },
        Item {
            color: Color::RED,
            title: String::from("RED"),
            state: State::Normal,
            rectangle: Rectangle::default(),
        },
        Item {
            color: Color::GOLD,
            title: String::from("GOLD"),
            state: State::Normal,
            rectangle: Rectangle::default(),
        },
        Item {
            color: Color::LIME,
            title: String::from("LIME"),
            state: State::Normal,
            rectangle: Rectangle::default(),
        },
        Item {
            color: Color::BLUE,
            title: String::from("BLUE"),
            state: State::Normal,
            rectangle: Rectangle::default(),
        },
        Item {
            color: Color::VIOLET,
            title: String::from("VIOLET"),
            state: State::Normal,
            rectangle: Rectangle::default(),
        },
        Item {
            color: Color::BROWN,
            title: String::from("BROWN"),
            state: State::Normal,
            rectangle: Rectangle::default(),
        },
        Item {
            color: Color::LIGHTGRAY,
            title: String::from("LIGHTGRAY"),
            state: State::Normal,
            rectangle: Rectangle::default(),
        },
        Item {
            color: Color::PINK,
            title: String::from("PINK"),
            state: State::Normal,
            rectangle: Rectangle::default(),
        },
        Item {
            color: Color::YELLOW,
            title: String::from("YELLOW"),
            state: State::Normal,
            rectangle: Rectangle::default(),
        },
        Item {
            color: Color::GREEN,
            title: String::from("GREEN"),
            state: State::Normal,
            rectangle: Rectangle::default(),
        },
        Item {
            color: Color::SKYBLUE,
            title: String::from("SKYBLUE"),
            state: State::Normal,
            rectangle: Rectangle::default(),
        },
        Item {
            color: Color::PURPLE,
            title: String::from("PURPLE"),
            state: State::Normal,
            rectangle: Rectangle::default(),
        },
        Item {
            color: Color::BEIGE,
            title: String::from("BEIGE"),
            state: State::Normal,
            rectangle: Rectangle::default(),
        },
    ];
    while !window.should_close() {
        // Updates
        items.iter_mut().enumerate().for_each(|(index, item)| {
            item.state = if check_point_rectangle(window.get_mouse_position(), item.rectangle) {
                State::Hover
            } else {
                State::Normal
            };
            item.rectangle.x = 20.0 + 100.0 * (index % 7) as f32 + 10.0 * (index % 7) as f32;
            item.rectangle.y = 80.0 + 100.0 * (index / 7) as f32 + 10.0 * (index / 7) as f32;
            item.rectangle.width = 100.0;
            item.rectangle.height = 100.0;
        });
        // Draws
        window.draw(|canvas| {
            canvas.clear_background(Color::RAYWHITE);
            canvas.draw_text("Color palette", 20, 30, 20, Color::DARKGRAY);
            items.iter_mut().for_each(|item| {
                canvas.draw_rectangle_rec(
                    item.rectangle,
                    item.color
                        .fade(if item.state == State::Hover { 0.6 } else { 1.0 }),
                );
                if window.is_key_down(Key::Space) || item.state == State::Hover {
                    canvas.draw_rectangle_lines_ex(item.rectangle, 6, Color::BLACK.fade(0.6));
                    canvas.draw_rectangle(
                        item.rectangle.x as i32,
                        item.rectangle.y as i32 + item.rectangle.height as i32 - 26,
                        item.rectangle.width as i32,
                        20,
                        Color::BLACK,
                    );
                    canvas.draw_text(
                        &item.title,
                        item.rectangle.x as i32 + (item.rectangle.width / 2.0) as i32
                            - measure_text(&item.title, 10) / 2,
                        item.rectangle.y as i32 + item.rectangle.height as i32 - 20,
                        10,
                        Color::WHITE,
                    );
                }
            });
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