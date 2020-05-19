use rayquaza::{color::Color, result::Result, window::WindowBuilder};

fn main() -> Result {
    let window = WindowBuilder::new()
        .title("Draw texture")
        .resizable()
        .vsync()
        .build()?;
    let texture = window.load_texture("logo/256x256.png")?;
    while !window.should_close() {
        window.draw(|canvas| {
            canvas.clear_background(Color::WHITE);
            canvas.draw_texture(
                &texture,
                window.width() / 2 - texture.width() / 2,
                window.height() / 2 - texture.height() / 2,
                Color::WHITE,
            );
        });
    }
    Ok(())
}
