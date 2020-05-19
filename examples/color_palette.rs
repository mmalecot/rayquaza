use rayquaza::{
    collision::check_point_rectangle, color::Color, input::Key, math::Rectangle, result::Result,
    window::WindowBuilder,
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

macro_rules! item {
    ($color:ident) => {
        Item {
            color: Color::$color,
            title: String::from(stringify!($color)),
            state: State::Normal,
            rectangle: Rectangle::default(),
        }
    };
}

macro_rules! items {
    ($($color:ident),*) => {
        vec![$(item!($color)),*]
    };
}

fn main() -> Result {
    let window = WindowBuilder::new()
        .title("Color palette")
        .vsync()
        .build()?;
    let mut items = items![
        DARKGRAY, MAROON, ORANGE, DARKGREEN, DARKBLUE, DARKPURPLE, DARKBROWN, GRAY, RED, GOLD,
        LIME, BLUE, VIOLET, BROWN, LIGHTGRAY, PINK, YELLOW, GREEN, SKYBLUE, PURPLE, BEIGE
    ];
    while !window.should_close() {
        items.iter_mut().enumerate().for_each(|(index, item)| {
            item.state = if check_point_rectangle(window.mouse_position(), item.rectangle) {
                State::Hover
            } else {
                State::Normal
            };
            item.rectangle.x = 20.0 + 100.0 * (index % 7) as f32 + 10.0 * (index % 7) as f32;
            item.rectangle.y = 80.0 + 100.0 * (index / 7) as f32 + 10.0 * (index / 7) as f32;
            item.rectangle.width = 100.0;
            item.rectangle.height = 100.0;
        });
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
                            - window.measure_text(&item.title, 10) / 2,
                        item.rectangle.y as i32 + item.rectangle.height as i32 - 20,
                        10,
                        Color::WHITE,
                    );
                }
            });
            canvas.draw_text(
                "Press space key to see all colors",
                window.width() - 200,
                window.height() - 40,
                10,
                Color::GRAY,
            );
        });
    }
    Ok(())
}
