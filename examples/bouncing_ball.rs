use rayquaza::{clamp, Color, Key, Result, Vector2, WindowBuilder};

struct Ball {
    position: Vector2,
    direction: Vector2,
    radius: f32,
}

const BALL_SPEED: f32 = 64.0;

fn main() -> Result {
    let window = WindowBuilder::new()
        .title("Bouncing ball")
        .vsync()
        .msaa_4x()
        .build()?;
    let mut pause = true;
    let mut ball = Ball {
        position: Vector2::new(
            window.get_screen_width() as f32 / 2.0,
            window.get_screen_height() as f32 / 2.0,
        ),
        direction: Vector2::new(5.0, 4.0),
        radius: 20.0,
    };
    while !window.should_close() {
        // Updates
        let delta = window.get_frame_time();
        if window.is_key_pressed(Key::Space) {
            pause = !pause;
        }
        if !pause {
            ball.position.x = clamp(
                ball.direction.x * BALL_SPEED * delta + ball.position.x,
                ball.radius,
                window.get_screen_width() as f32 - ball.radius,
            );
            ball.position.y = clamp(
                ball.direction.y * BALL_SPEED * delta + ball.position.y,
                ball.radius,
                window.get_screen_height() as f32 - ball.radius,
            );
            if ball.position.x >= window.get_screen_width() as f32 - ball.radius
                || ball.position.x <= ball.radius
            {
                ball.direction.x *= -1.0;
            }
            if ball.position.y >= window.get_screen_height() as f32 - ball.radius
                || ball.position.y <= ball.radius
            {
                ball.direction.y *= -1.0;
            }
        }
        // Draws
        window.draw(|canvas| {
            canvas.clear_background(Color::BLACK);
            canvas.draw_circle_vec(ball.position, ball.radius, Color::MAROON);
            canvas.draw_text(
                "PRESS SPACE to PAUSE BALL MOVEMENT",
                10,
                window.get_screen_height() - 25,
                20,
                Color::LIGHTGRAY,
            );
            if pause {
                canvas.draw_text("PAUSED", 339, 200, 30, Color::GRAY);
            }
            canvas.draw_fps(10, 10);
        });
    }
    Ok(())
}
