use rayquaza::{
    color::Color,
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
    let mut ball = Ball {
        position: Vector2::new(window.width() as f32 / 2.0, window.height() as f32 / 2.0),
        velocity: Vector2::new(320.0, 256.0),
        radius: 20.0,
    };
    while !window.should_close() {
        ball.position.x = clamp(
            ball.position.x + ball.velocity.x * window.frame_time(),
            ball.radius,
            window.width() as f32 - ball.radius,
        );
        ball.position.y = clamp(
            ball.position.y + ball.velocity.y * window.frame_time(),
            ball.radius,
            window.height() as f32 - ball.radius,
        );
        if ball.position.x >= window.width() as f32 - ball.radius || ball.position.x <= ball.radius
        {
            ball.velocity.x *= -1.0;
        }
        if ball.position.y >= window.height() as f32 - ball.radius || ball.position.y <= ball.radius
        {
            ball.velocity.y *= -1.0;
        }
        window.draw(|canvas| {
            canvas.clear_background(Color::BLACK);
            canvas.draw_circle_vec(ball.position, ball.radius, Color::MAROON);
        });
    }
    Ok(())
}
