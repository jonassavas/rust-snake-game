use macroquad::prelude::*;

mod snake;
mod grid;

use snake::{Snake, Direction};
use grid::{Grid, GRID_WIDTH, GRID_HEIGHT};

// Generate random food position
fn random_food() -> (i32, i32) {
    (
        macroquad::rand::gen_range(0, GRID_WIDTH),
        macroquad::rand::gen_range(0, GRID_HEIGHT),
    )
}

#[macroquad::main("Snake")]
async fn main() {
    let mut snake = Snake::new(10, 10);
    let mut food = random_food();

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

            if snake.step(GRID_WIDTH, GRID_HEIGHT) {
                println!("Game Over!");
                break;
            } 

            // --- Food collision ---
            if snake.head_position() == food {
                snake.grow();
                food = random_food();
            }
        }

        // --- Grid ---
        let grid = Grid::compute();
        grid.draw();

        // --- Draw snake ---
        for (i, (x, y)) in snake.body.iter().enumerate() {
            let (px, py) = grid.to_screen(*x as f32, *y as f32);

            let color = if i == 0 { GREEN } else { DARKGREEN };

            draw_rectangle(
                px + 2.0,
                py + 2.0,
                grid.cell_size - 4.0,
                grid.cell_size - 4.0,
                color,
            );
        }

        // --- Draw food ---
        let (fx, fy) = grid.to_screen(food.0 as f32, food.1 as f32);

        draw_rectangle(
            fx + 2.0,
            fy + 2.0,
            grid.cell_size - 4.0,
            grid.cell_size - 4.0,
            RED,
        );

        next_frame().await;
    }
}