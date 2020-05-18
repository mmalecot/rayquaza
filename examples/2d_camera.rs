use rayquaza::{
    camera::Camera2D,
    color::Color,
    input::{Key, MouseButton},
    math::{clamp, Rectangle, Vector2},
    misc::get_random_value,
    result::Result,
    window::WindowBuilder,
};

pub const WORLD_WIDTH: i32 = 30000;
pub const BUILDING_COUNT: i32 = WORLD_WIDTH / BUILDING_WIDTH;
pub const BUILDING_WIDTH: i32 = 100;
pub const PLAYER_SPEED: f32 = 240.0;
pub const ROTATION_SPEED: f32 = 100.0;

struct Building {
    rectangle: Rectangle,
    color: Color,
}

fn main() -> Result {
    let window = WindowBuilder::new()
        .title("2D camera")
        .resizable()
        .vsync()
        .msaa_4x()
        .build()?;
    let mut player = Rectangle::new(WORLD_WIDTH as f32 / 2.0, 0.0, 40.0, 40.0);
    let mut buildings = Vec::with_capacity(BUILDING_COUNT as usize);
    for index in 0..BUILDING_COUNT {
        let height = get_random_value(100, 800);
        let rectangle = Rectangle::new(
            index as f32 * BUILDING_WIDTH as f32,
            player.height - height as f32,
            BUILDING_WIDTH as f32,
            height as f32,
        );
        let color = Color::new(
            get_random_value(127, 220) as u8,
            get_random_value(127, 220) as u8,
            get_random_value(127, 220) as u8,
            255,
        );
        buildings.push(Building { rectangle, color });
    }
    let mut camera = Camera2D::new(
        Vector2::new(
            window.get_width() as f32 / 2.0,
            window.get_height() as f32 / 2.0,
        ),
        Vector2::new(
            player.x + player.width / 2.0,
            player.y + player.height / 2.0,
        ),
        0.0,
        1.0,
    );
    while !window.should_close() {
        if window.is_key_down(Key::Right) {
            player.x = clamp(
                player.x as f32 + PLAYER_SPEED * window.get_frame_time(),
                0.0,
                WORLD_WIDTH as f32 - player.width,
            );
        }
        if window.is_key_down(Key::Left) {
            player.x = clamp(
                player.x as f32 - PLAYER_SPEED * window.get_frame_time(),
                0.0,
                WORLD_WIDTH as f32 - player.height,
            );
        }
        if window.is_mouse_button_down(MouseButton::Left) {
            camera.rotation += ROTATION_SPEED * window.get_frame_time();
        }
        if window.is_mouse_button_down(MouseButton::Right) {
            camera.rotation -= ROTATION_SPEED * window.get_frame_time();
        }
        if window.is_key_pressed(Key::R) {
            camera.zoom = 1.0;
            camera.rotation = 0.0;
        }
        camera.offset = Vector2::new(
            window.get_width() as f32 / 2.0,
            window.get_height() as f32 / 2.0,
        );
        camera.target = Vector2::new(
            player.x + player.width / 2.0,
            player.y + player.height / 2.0,
        );
        camera.zoom = clamp(
            camera.zoom + camera.zoom * window.get_mouse_wheel_move() as f32 * 0.25,
            0.1,
            3.0,
        );
        window.draw(|canvas| {
            canvas.clear_background(Color::SKYBLUE);
            canvas.mode_2d(camera, |canvas| {
                canvas.draw_rectangle(0, player.height as i32, WORLD_WIDTH, 20000, Color::DARKGRAY);
                buildings.iter().for_each(|building| {
                    canvas.draw_rectangle_rec(building.rectangle, building.color);
                });
                canvas.draw_rectangle_rec(player, Color::RED);
            });
            canvas.draw_rectangle(10, 10, 250, 115, Color::DARKGREEN.fade(0.5));
            canvas.draw_text("Controls:", 20, 20, 10, Color::WHITE);
            canvas.draw_text("- Right / Left keys to move", 40, 40, 10, Color::WHITE);
            canvas.draw_text("- Mouse wheel to zoom in-out", 40, 60, 10, Color::WHITE);
            canvas.draw_text("- Mouse buttons to rotate", 40, 80, 10, Color::WHITE);
            canvas.draw_text(
                "- R key to reset zoom and rotation",
                40,
                100,
                10,
                Color::WHITE,
            );
        });
    }
    Ok(())
}
