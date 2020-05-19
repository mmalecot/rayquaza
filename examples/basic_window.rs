use rayquaza::{color::Color, result::Result, window::WindowBuilder};

fn main() -> Result {
    let window = WindowBuilder::new()
        .title("Basic window")
        .resizable()
        .vsync()
        .build()?;
    let text = "Congrats! You created your first window!";
    let size = 20;
    let width = window.measure_text(text, size);
    while !window.should_close() {
        window.draw(|canvas| {
            canvas.clear_background(Color::BLACK);
            canvas.draw_text(
                text,
                window.width() / 2 - width / 2,
                window.height() / 2 - size / 2,
                size,
                Color::WHITE,
            );
        });
    }
    Ok(())
}
