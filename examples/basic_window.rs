use rayquaza::{color::Color, result::Result, text::measure_text, window::WindowBuilder};

const TEXT: &str = "Congrats! You created your first window!";
const TEXT_SIZE: i32 = 20;

fn main() -> Result {
    let window = WindowBuilder::new()
        .title("Basic window")
        .resizable()
        .vsync()
        .build()?;
    while !window.should_close() {
        window.draw(|canvas| {
            canvas.clear_background(Color::BLACK);
            canvas.draw_text(
                TEXT,
                window.get_width() / 2 - measure_text(TEXT, TEXT_SIZE) / 2,
                window.get_height() / 2 - TEXT_SIZE / 2,
                TEXT_SIZE,
                Color::WHITE,
            );
        });
    }
    Ok(())
}
