use rayquaza::{color::Color, math::Vector2, result::Result, window::WindowBuilder};

fn main() -> Result {
    let window = WindowBuilder::new()
        .title("Draw font")
        .resizable()
        .vsync()
        .build()?;
    let font = window.load_font_ex("resources/pacifico.ttf", 100)?;
    let text = "Hello world!";
    let size = window.measure_text_ex(&font, text, 100.0, 4.0);
    while !window.should_close() {
        window.draw(|canvas| {
            canvas.clear_background(Color::WHITE);
            canvas.draw_text_ex(
                &font,
                text,
                Vector2::new(
                    window.get_width() as f32 / 2.0 - size.x / 2.0,
                    window.get_height() as f32 / 2.0 - size.y / 2.0,
                ),
                100.0,
                4.0,
                Color::RED,
            );
        });
    }
    Ok(())
}
