use rayquaza::{
    color::Color,
    input::Key,
    math::{clamp, Vector2},
    result::Result,
    window::WindowBuilder,
};

struct Ball {
    position: Vector2,
    velocity: Vector2,
    radius: f32,
}

fn main() -> Result {
    let window = WindowBuilder::new()
        .title("Bouncing ball")
        .vsync()
        .msaa_4x()
        .build()?;
    let mut pause = true;
    let mut ball = Ball {
        position: Vector2::new(
            window.get_width() as f32 / 2.0,
            window.get_height() as f32 / 2.0,
        ),
        velocity: Vector2::new(320.0, 256.0),
        radius: 20.0,
    };
    while !window.should_close() {
        // Updates
        if window.is_key_pressed(Key::Space) {
            pause = !pause;
        }
        if !pause {
            ball.position.x = clamp(
                ball.position.x + ball.velocity.x * window.get_frame_time(),
                ball.radius,
                window.get_width() as f32 - ball.radius,
            );
            ball.position.y = clamp(
                ball.position.y + ball.velocity.y * window.get_frame_time(),
                ball.radius,
                window.get_height() as f32 - ball.radius,
            );
            if ball.position.x >= window.get_width() as f32 - ball.radius
                || ball.position.x <= ball.radius
            {
                ball.velocity.x *= -1.0;
            }
            if ball.position.y >= window.get_height() as f32 - ball.radius
                || ball.position.y <= ball.radius
            {
                ball.velocity.y *= -1.0;
            }
        }
        // Draws
        window.draw(|canvas| {
            canvas.clear_background(Color::BLACK);
            canvas.draw_circle_vec(ball.position, ball.radius, Color::MAROON);
            canvas.draw_text(
                "PRESS SPACE to PAUSE BALL MOVEMENT",
                10,
                window.get_height() - 25,
                20,
                Color::LIGHTGRAY,
            );
            if pause {
                canvas.draw_text("PAUSED", 339, 200, 30, Color::GRAY);
            }
        });
    }
    Ok(())
}
