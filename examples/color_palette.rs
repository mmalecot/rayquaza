use rayquaza::{
    collision::check_point_rectangle, color::Color, input::Key, math::Rectangle, result::Result,
    text::measure_text, window::WindowBuilder,
};

#[derive(PartialEq)]
enum State {
    Normal,
    Hover,
}

struct Item<'s> {
    color: Color,
    title: &'s str,
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
            title: "DARKGRAY",
            state: State::Normal,
            rectangle: Rectangle::default(),
        },
        Item {
            color: Color::MAROON,
            title: "MAROON",
            state: State::Normal,
            rectangle: Rectangle::default(),
        },
        Item {
            color: Color::ORANGE,
            title: "ORANGE",
            state: State::Normal,
            rectangle: Rectangle::default(),
        },
        Item {
            color: Color::DARKGREEN,
            title: "DARKGREEN",
            state: State::Normal,
            rectangle: Rectangle::default(),
        },
        Item {
            color: Color::DARKBLUE,
            title: "DARKBLUE",
            state: State::Normal,
            rectangle: Rectangle::default(),
        },
        Item {
            color: Color::DARKPURPLE,
            title: "DARKPURPLE",
            state: State::Normal,
            rectangle: Rectangle::default(),
        },
        Item {
            color: Color::DARKBROWN,
            title: "DARKBROWN",
            state: State::Normal,
            rectangle: Rectangle::default(),
        },
        Item {
            color: Color::GRAY,
            title: "GRAY",
            state: State::Normal,
            rectangle: Rectangle::default(),
        },
        Item {
            color: Color::RED,
            title: "RED",
            state: State::Normal,
            rectangle: Rectangle::default(),
        },
        Item {
            color: Color::GOLD,
            title: "GOLD",
            state: State::Normal,
            rectangle: Rectangle::default(),
        },
        Item {
            color: Color::LIME,
            title: "LIME",
            state: State::Normal,
            rectangle: Rectangle::default(),
        },
        Item {
            color: Color::BLUE,
            title: "BLUE",
            state: State::Normal,
            rectangle: Rectangle::default(),
        },
        Item {
            color: Color::VIOLET,
            title: "VIOLET",
            state: State::Normal,
            rectangle: Rectangle::default(),
        },
        Item {
            color: Color::BROWN,
            title: "BROWN",
            state: State::Normal,
            rectangle: Rectangle::default(),
        },
        Item {
            color: Color::LIGHTGRAY,
            title: "LIGHTGRAY",
            state: State::Normal,
            rectangle: Rectangle::default(),
        },
        Item {
            color: Color::PINK,
            title: "PINK",
            state: State::Normal,
            rectangle: Rectangle::default(),
        },
        Item {
            color: Color::YELLOW,
            title: "YELLOW",
            state: State::Normal,
            rectangle: Rectangle::default(),
        },
        Item {
            color: Color::GREEN,
            title: "GREEN",
            state: State::Normal,
            rectangle: Rectangle::default(),
        },
        Item {
            color: Color::SKYBLUE,
            title: "SKYBLUE",
            state: State::Normal,
            rectangle: Rectangle::default(),
        },
        Item {
            color: Color::PURPLE,
            title: "PURPLE",
            state: State::Normal,
            rectangle: Rectangle::default(),
        },
        Item {
            color: Color::BEIGE,
            title: "BEIGE",
            state: State::Normal,
            rectangle: Rectangle::default(),
        },
    ];
    items.iter_mut().enumerate().for_each(|(index, item)| {
        item.rectangle.x = 20.0 + 100.0 * (index % 7) as f32 + 10.0 * (index % 7) as f32;
        item.rectangle.y = 80.0 + 100.0 * (index / 7) as f32 + 10.0 * (index / 7) as f32;
        item.rectangle.width = 100.0;
        item.rectangle.height = 100.0;
    });
    while !window.should_close() {
        // Updates
        items.iter_mut().for_each(|item| {
            item.state = if check_point_rectangle(window.get_mouse_position(), item.rectangle) {
                State::Hover
            } else {
                State::Normal
            }
        });
        // Draws
        window.draw(|canvas| {
            canvas.clear_background(Color::RAYWHITE);
            canvas.draw_text("Color palette", 20, 30, 20, Color::BLACK);
            items.iter_mut().for_each(|item| {
                canvas.draw_rectangle_rec(
                    item.rectangle,
                    item.color
                        .fade(if item.state == State::Hover { 0.6 } else { 1.0 }),
                );
                if window.is_key_down(Key::Space) || item.state == State::Hover {
                    canvas.draw_rectangle(
                        item.rectangle.x as i32,
                        item.rectangle.y as i32 + item.rectangle.height as i32 - 26,
                        item.rectangle.width as i32,
                        20,
                        Color::BLACK,
                    );
                    canvas.draw_rectangle_lines_ex(item.rectangle, 6, Color::BLACK.fade(0.3));
                    canvas.draw_text(
                        item.title,
                        item.rectangle.x as i32 + item.rectangle.width as i32
                            - measure_text(item.title, 10)
                            - 12,
                        item.rectangle.y as i32 + item.rectangle.height as i32 - 20,
                        10,
                        item.color,
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
