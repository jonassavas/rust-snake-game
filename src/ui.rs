use macroquad::prelude::*;

pub fn draw_menu() {
    draw_text("SNAKE", 300.0, 200.0, 60.0, GREEN);

    draw_text(
        "Press ENTER to start",
        260.0,
        300.0,
        30.0,
        WHITE,
    );
}

pub fn draw_game_over(score: i32) {
    draw_text("GAME OVER", 260.0, 200.0, 60.0, RED);

    draw_text(
        &format!("Final Score: {}", score),
        260.0,
        260.0,
        30.0,
        WHITE,
    );

    draw_text(
        "Press R to restart",
        260.0,
        320.0,
        30.0,
        WHITE,
    );

    draw_text(
        "Press ESC for menu",
        260.0,
        360.0,
        30.0,
        WHITE,
    );
}

pub fn draw_score(score: i32) {
    draw_text(
        &format!("Score: {}", score),
        20.0,
        40.0,
        30.0,
        WHITE,
    );
}

pub fn draw_fps() {
    let fps_text = format!("FPS: {}", get_fps());

    let text_dimensions =
        measure_text(&fps_text, None, 24, 1.0);

    draw_text(
        &fps_text,
        screen_width() - text_dimensions.width - 20.0,
        30.0,
        24.0,
        GRAY,
    );
}


pub fn draw_pause_menu() {
    draw_text("PAUSED", 280.0, 200.0, 60.0, YELLOW);

    draw_text(
        "Press ESC to resume",
        240.0,
        300.0,
        30.0,
        WHITE,
    );

    draw_text(
        "Press M for menu",
        260.0,
        350.0,
        30.0,
        WHITE,
    );
}