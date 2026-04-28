use macroquad::prelude::*;

mod snake;
mod grid;

use snake::{Snake, Direction};
use grid::{Grid, GRID_WIDTH, GRID_HEIGHT};

#[derive(PartialEq)]
enum GameState {
    Menu,
    Playing,
    GameOver,
}

// Generate random food position
fn random_food(snake: &Snake) -> (i32, i32) {
    loop {
        let pos = (
            macroquad::rand::gen_range(0, GRID_WIDTH),
            macroquad::rand::gen_range(0, GRID_HEIGHT),
        );

        // If position is NOT inside snake, return it
        if !snake.body.contains(&pos) {
            return pos;
        }
    }
}

fn reset_game() -> (Snake, (i32, i32), Direction, f32, i32) {
    let snake = Snake::new(10, 10);
    let food = random_food(&snake);
    let next_direction = Direction::Right;
    let move_timer = 0.0;
    let score = 0;

    (snake, food, next_direction, move_timer, score)
}

#[macroquad::main("Snake")]
async fn main() {
    let mut state = GameState::Menu;

    let (mut snake, mut food, mut next_direction, mut move_timer, mut score) = reset_game(); 


    loop {
        clear_background(Color::from_rgba(20, 20, 20, 255));

        match state {
            GameState::Menu => {
                draw_text("SNAKE", 300.0, 200.0, 60.0, GREEN);
                draw_text("Press ENTER to start", 260.0, 300.0, 30.0, WHITE);

                if is_key_pressed(KeyCode::Enter) {
                    let (s, f, d, t, sc) = reset_game();
                    snake = s;
                    food = f;
                    next_direction = d;
                    move_timer = t;
                    score = sc;

                    state = GameState::Playing;
                }
            }

            GameState::Playing => {
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

                // --- Speed scaling ---
                let base_delay = 0.12;
                let move_delay = (base_delay - score as f32 * 0.005).max(0.04);

                move_timer += get_frame_time();

                while move_timer >= move_delay {
                    move_timer -= move_delay;

                    snake.update_direction(next_direction);

                    if snake.step(GRID_WIDTH, GRID_HEIGHT) {
                        state = GameState::GameOver;
                        break;
                    }

                    if snake.head_position() == food {
                        snake.grow();
                        food = random_food(&snake);
                        score += 1;
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

                // --- Score ---
                draw_text(
                    &format!("Score: {}", score),
                    20.0,
                    40.0,
                    30.0,
                    WHITE,
                );
            }

            GameState::GameOver => {
                draw_text("GAME OVER", 260.0, 200.0, 60.0, RED);
                draw_text(
                    &format!("Final Score: {}", score),
                    260.0,
                    260.0,
                    30.0,
                    WHITE,
                );
                draw_text("Press R to restart", 260.0, 320.0, 30.0, WHITE);
                draw_text("Press ESC for menu", 260.0, 360.0, 30.0, WHITE);

                if is_key_pressed(KeyCode::R) {
                    let (s, f, d, t, sc) = reset_game();
                    snake = s;
                    food = f;
                    next_direction = d;
                    move_timer = t;
                    score = sc;

                    state = GameState::Playing;
                }

                if is_key_pressed(KeyCode::Escape) {
                    state = GameState::Menu;
                }
            }
        }

        next_frame().await;
    }
}