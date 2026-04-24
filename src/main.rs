use macroquad::prelude::*;

mod snake;
mod grid;

use snake::{Snake, Direction};
use grid::Grid;

#[macroquad::main("Snake")]
async fn main() {
    let mut snake = Snake::new(10, 10);

    let mut prev_x = snake.x;
    let mut prev_y = snake.y;
    let mut next_direction = snake.direction;

    let mut move_timer = 0.0;
    let move_delay = 0.08;

    loop {
        clear_background(Color::from_rgba(20, 20, 20, 255));

        // --- Input ---
        if is_key_pressed(KeyCode::Up) {
            next_direction = Direction::Up;
        }
        if is_key_pressed(KeyCode::Down) {
            next_direction = Direction::Down;
        }
        if is_key_pressed(KeyCode::Left) {
            next_direction = Direction::Left;
        }
        if is_key_pressed(KeyCode::Right) {
            next_direction = Direction::Right;
        }

        // --- Movement ---
        move_timer += get_frame_time();

        while move_timer >= move_delay {
            move_timer -= move_delay;

            snake.update_direction(next_direction);

            prev_x = snake.x;
            prev_y = snake.y;

            snake.step();
        }

        // --- Grid ---
        let grid = Grid::compute();
        grid.draw();

        // --- Interpolation ---
        let t = (move_timer / move_delay).min(1.0);

        let interp_x = prev_x as f32 + (snake.x - prev_x) as f32 * t;
        let interp_y = prev_y as f32 + (snake.y - prev_y) as f32 * t;

        let (px, py) = grid.to_screen(interp_x, interp_y);

        draw_rectangle(
            px + 2.0,
            py + 2.0,
            grid.cell_size - 4.0,
            grid.cell_size - 4.0,
            GREEN,
        );

        next_frame().await;
    }
}