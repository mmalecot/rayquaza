use rayquaza::{Color, Result, WindowBuilder};

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
                window.get_width() / 2 - texture.get_width() / 2,
                window.get_height() / 2 - texture.get_height() / 2,
                Color::WHITE,
            );
        });
    }
    Ok(())
}
