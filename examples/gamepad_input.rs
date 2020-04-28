use rayquaza::{
    color::Color,
    drawing::Canvas,
    input::{Gamepad, GamepadAxis, GamepadButton},
    result::Result,
    window::{Window, WindowBuilder},
};

fn main() -> Result {
    let window = WindowBuilder::new()
        .title("Gamepad input")
        .resizable()
        .vsync()
        .msaa_4x()
        .build()?;
    let axis = vec![
        GamepadAxis::LeftX,
        GamepadAxis::LeftY,
        GamepadAxis::RightX,
        GamepadAxis::RightY,
        GamepadAxis::LeftTrigger,
        GamepadAxis::RightTrigger,
    ];
    while !window.should_close() {
        // Draws
        window.draw(|mut canvas| {
            canvas.clear_background(Color::RAYWHITE);
            if window.is_gamepad_available(Gamepad::One) {
                draw_gamepad(
                    &window,
                    &mut canvas,
                    (window.get_width() as f32 / 2.0) as i32 - 160,
                    (window.get_height() as f32 / 2.0) as i32 - 105,
                );
            }
            canvas.draw_text(
                &format!(
                    "GP1: {}",
                    window
                        .get_gamepad_name(Gamepad::One)
                        .unwrap_or_else(|| String::from("None"))
                ),
                10,
                10,
                20,
                Color::DARKGRAY,
            );
            canvas.draw_text("Axis:", 10, 60, 20, Color::DARKGRAY);
            axis.iter().enumerate().for_each(|(index, axis)| {
                canvas.draw_text(
                    &format!(
                        "{:?}: {:.2}",
                        axis.clone(),
                        window.get_gamepad_axis_movement(Gamepad::One, axis.clone())
                    ),
                    10,
                    110 + (index as i32 * 20),
                    10,
                    Color::DARKGRAY,
                );
            });
        });
    }
    Ok(())
}

fn draw_gamepad(window: &Window, canvas: &mut Canvas, x: i32, y: i32) {
    // Skeleton
    canvas.draw_triangle(
        (x as f32 + 30.0, y as f32 + 200.0),
        (x as f32, y as f32 + 150.0),
        (x as f32, y as f32 + 180.0),
        Color::LIGHTGRAY,
    );
    canvas.draw_triangle(
        (x as f32 + 90.0, y as f32 + 150.0),
        (x as f32, y as f32 + 150.0),
        (x as f32 + 30.0, y as f32 + 200.0),
        Color::LIGHTGRAY,
    );
    canvas.draw_triangle(
        (x as f32 + 90.0, y as f32 + 150.0),
        (x as f32 + 45.0, y as f32 + 25.0),
        (x as f32, y as f32 + 150.0),
        Color::LIGHTGRAY,
    );
    canvas.draw_triangle(
        (x as f32 + 90.0, y as f32 + 150.0),
        (x as f32 + 90.0, y as f32 + 10.0),
        (x as f32 + 45.0, y as f32 + 25.0),
        Color::LIGHTGRAY,
    );
    canvas.draw_rectangle(x + 90, y + 10, 140, 140, Color::LIGHTGRAY);
    canvas.draw_triangle(
        (x as f32 + 275.0, y as f32 + 25.0),
        (x as f32 + 230.0, y as f32 + 10.0),
        (x as f32 + 230.0, y as f32 + 150.0),
        Color::LIGHTGRAY,
    );
    canvas.draw_triangle(
        (x as f32 + 320.0, y as f32 + 150.0),
        (x as f32 + 275.0, y as f32 + 25.0),
        (x as f32 + 230.0, y as f32 + 150.0),
        Color::LIGHTGRAY,
    );
    canvas.draw_triangle(
        (x as f32 + 320.0, y as f32 + 150.0),
        (x as f32 + 230.0, y as f32 + 150.0),
        (x as f32 + 290.0, y as f32 + 200.0),
        Color::LIGHTGRAY,
    );
    canvas.draw_triangle(
        (x as f32 + 320.0, y as f32 + 180.0),
        (x as f32 + 320.0, y as f32 + 150.0),
        (x as f32 + 290.0, y as f32 + 200.0),
        Color::LIGHTGRAY,
    );

    // Left joystick
    canvas.draw_circle(x + 90, y + 60, 16.0, Color::DARKGRAY);
    if window.is_gamepad_button_down(Gamepad::One, GamepadButton::MiddleLeftThumb) {
        canvas.draw_circle(
            x + 90
                + (window.get_gamepad_axis_movement(Gamepad::One, GamepadAxis::LeftX) * 8.0) as i32,
            y + 60
                + (window.get_gamepad_axis_movement(Gamepad::One, GamepadAxis::LeftY) * 8.0) as i32,
            11.0,
            Color::BLACK,
        );
        canvas.draw_circle(
            x + 90
                + (window.get_gamepad_axis_movement(Gamepad::One, GamepadAxis::LeftX) * 8.0) as i32,
            y + 60
                + (window.get_gamepad_axis_movement(Gamepad::One, GamepadAxis::LeftY) * 8.0) as i32,
            9.0,
            Color::GRAY,
        );
    } else {
        canvas.draw_circle(
            x + 90
                + (window.get_gamepad_axis_movement(Gamepad::One, GamepadAxis::LeftX) * 8.0) as i32,
            y + 60
                + (window.get_gamepad_axis_movement(Gamepad::One, GamepadAxis::LeftY) * 8.0) as i32,
            12.0,
            Color::BLACK,
        );
        canvas.draw_circle(
            x + 90
                + (window.get_gamepad_axis_movement(Gamepad::One, GamepadAxis::LeftX) * 8.0) as i32,
            y + 60
                + (window.get_gamepad_axis_movement(Gamepad::One, GamepadAxis::LeftY) * 8.0) as i32,
            10.0,
            Color::DARKGRAY,
        );
    }

    // D-Pad
    canvas.draw_rectangle(x + 100, y + 100, 36, 12, Color::BLACK);
    canvas.draw_rectangle(x + 112, y + 88, 12, 36, Color::BLACK);
    if window.is_gamepad_button_down(Gamepad::One, GamepadButton::LeftFaceLeft) {
        canvas.draw_rectangle(x + 100, y + 100, 12, 12, Color::GRAY);
    }
    if window.is_gamepad_button_down(Gamepad::One, GamepadButton::LeftFaceRight) {
        canvas.draw_rectangle(x + 124, y + 100, 12, 12, Color::GRAY);
    }
    if window.is_gamepad_button_down(Gamepad::One, GamepadButton::LeftFaceUp) {
        canvas.draw_rectangle(x + 112, y + 88, 12, 12, Color::GRAY);
    }
    if window.is_gamepad_button_down(Gamepad::One, GamepadButton::LeftFaceDown) {
        canvas.draw_rectangle(x + 112, y + 112, 12, 12, Color::GRAY);
    }

    // Right joystick
    canvas.draw_circle(x + 204, y + 104, 16.0, Color::DARKGRAY);
    if window.is_gamepad_button_down(Gamepad::One, GamepadButton::MiddleRightThumb) {
        canvas.draw_circle(
            x + 204
                + (window.get_gamepad_axis_movement(Gamepad::One, GamepadAxis::RightX) * 8.0)
                    as i32,
            y + 104
                + (window.get_gamepad_axis_movement(Gamepad::One, GamepadAxis::RightY) * 8.0)
                    as i32,
            11.0,
            Color::BLACK,
        );
        canvas.draw_circle(
            x + 204
                + (window.get_gamepad_axis_movement(Gamepad::One, GamepadAxis::RightX) * 8.0)
                    as i32,
            y + 104
                + (window.get_gamepad_axis_movement(Gamepad::One, GamepadAxis::RightY) * 8.0)
                    as i32,
            9.0,
            Color::GRAY,
        );
    } else {
        canvas.draw_circle(
            x + 204
                + (window.get_gamepad_axis_movement(Gamepad::One, GamepadAxis::RightX) * 8.0)
                    as i32,
            y + 104
                + (window.get_gamepad_axis_movement(Gamepad::One, GamepadAxis::RightY) * 8.0)
                    as i32,
            12.0,
            Color::BLACK,
        );
        canvas.draw_circle(
            x + 204
                + (window.get_gamepad_axis_movement(Gamepad::One, GamepadAxis::RightX) * 8.0)
                    as i32,
            y + 104
                + (window.get_gamepad_axis_movement(Gamepad::One, GamepadAxis::RightY) * 8.0)
                    as i32,
            10.0,
            Color::DARKGRAY,
        );
    }

    // Right buttons
    if window.is_gamepad_button_down(Gamepad::One, GamepadButton::RightFaceUp) {
        canvas.draw_circle(x + 240, y + 45, 7.0, Color::YELLOW);
    } else {
        canvas.draw_circle(x + 240, y + 45, 8.0, Color::BLACK);
    }
    if window.is_gamepad_button_down(Gamepad::One, GamepadButton::RightFaceDown) {
        canvas.draw_circle(x + 240, y + 75, 7.0, Color::GREEN);
    } else {
        canvas.draw_circle(x + 240, y + 75, 8.0, Color::BLACK);
    }
    if window.is_gamepad_button_down(Gamepad::One, GamepadButton::RightFaceLeft) {
        canvas.draw_circle(x + 225, y + 60, 7.0, Color::BLUE);
    } else {
        canvas.draw_circle(x + 225, y + 60, 8.0, Color::BLACK);
    }
    if window.is_gamepad_button_down(Gamepad::One, GamepadButton::RightFaceRight) {
        canvas.draw_circle(x + 255, y + 60, 7.0, Color::RED);
    } else {
        canvas.draw_circle(x + 255, y + 60, 8.0, Color::BLACK);
    }

    // Middle buttons
    if window.is_gamepad_button_down(Gamepad::One, GamepadButton::MiddleLeft) {
        canvas.draw_circle(x + 140, y + 60, 5.0, Color::GRAY);
    } else {
        canvas.draw_circle(x + 140, y + 60, 6.0, Color::DARKGRAY);
    }
    if window.is_gamepad_button_down(Gamepad::One, GamepadButton::Middle) {
        canvas.draw_circle(x + 165, y + 30, 9.0, Color::GRAY);
    } else {
        canvas.draw_circle(x + 165, y + 30, 10.0, Color::BLACK);
    }
    if window.is_gamepad_button_down(Gamepad::One, GamepadButton::MiddleRight) {
        canvas.draw_circle(x + 190, y + 60, 5.0, Color::GRAY);
    } else {
        canvas.draw_circle(x + 190, y + 60, 6.0, Color::DARKGRAY);
    }
}
