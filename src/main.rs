use macroquad::prelude::*;

mod snake;
mod grid;
mod particle;
mod ui;

use snake::{Snake, Direction};
use grid::{Grid, GRID_WIDTH, GRID_HEIGHT};
use particle::Particle;

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

fn window_conf() -> Conf {
    Conf {
        window_title: "Snake".to_owned(),
        window_width: 900,
        window_height: 900,
        high_dpi: true,
        fullscreen: false,
        sample_count: 1,
        window_resizable: true,

        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = GameState::Menu;

    let (mut snake, mut food, mut next_direction, mut move_timer, mut score) =
        reset_game();

    let mut prev_body = snake.body.clone();

    let head_color = Color::from_rgba(80, 220, 120, 255);
    let body_color = Color::from_rgba(40, 160, 80, 255);

    let mut particles: Vec<Particle> = Vec::new();

    loop {
        clear_background(Color::from_rgba(18, 18, 18, 255));

        match state {
            GameState::Menu => {
                ui::draw_menu(); 

                draw_text(
                    "Press ENTER to start",
                    260.0,
                    300.0,
                    30.0,
                    WHITE,
                );

                if is_key_pressed(KeyCode::Enter) {
                    let (s, f, d, t, sc) = reset_game();

                    snake = s;
                    food = f;
                    next_direction = d;
                    move_timer = t;
                    score = sc;

                    prev_body = snake.body.clone();

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
                let move_delay =
                    (base_delay - score as f32 * 0.005).max(0.04);

                move_timer += get_frame_time();

                while move_timer >= move_delay {
                    move_timer -= move_delay;

                    snake.update_direction(next_direction);

                    // Save previous positions BEFORE movement
                    prev_body = snake.body.clone();

                    if snake.step(GRID_WIDTH, GRID_HEIGHT) {
                        state = GameState::GameOver;
                        break;
                    }

                    // --- Food collision ---
                    if snake.head_position() == food {
                        snake.grow();
                        score += 1;

                        // Particle burst
                        let grid = Grid::compute();

                        let (fx, fy) =
                            grid.to_screen(food.0 as f32, food.1 as f32);

                        let center_x = fx + grid.cell_size / 2.0;
                        let center_y = fy + grid.cell_size / 2.0;

                        let speed = 180.0;
                        let offset = 10.0;

                        particles.push(Particle::new(
                            center_x - offset,
                            center_y - offset,
                            -speed,
                            -speed,
                        ));

                        particles.push(Particle::new(
                            center_x + offset,
                            center_y - offset,
                            speed,
                            -speed,
                        ));

                        particles.push(Particle::new(
                            center_x - offset,
                            center_y + offset,
                            -speed,
                            speed,
                        ));

                        particles.push(Particle::new(
                            center_x + offset,
                            center_y + offset,
                            speed,
                            speed,
                        ));

                        food = random_food(&snake);
                    }
                }

                // --- Update particles ---
                let dt = get_frame_time();

                for particle in particles.iter_mut() {
                    particle.update(dt);
                }

                particles.retain(|p| !p.is_dead());

                // --- Grid ---
                let grid = Grid::compute();
                grid.draw();

                // --- Fix growing interpolation ---
                while prev_body.len() < snake.body.len() {
                    prev_body.push(*prev_body.last().unwrap());
                }

                // --- Interpolation ---
                let t = (move_timer / move_delay).min(1.0);

                // --- Draw snake ---
                for (i, ((x, y), (px_old, py_old))) in snake
                    .body
                    .iter()
                    .zip(prev_body.iter())
                    .enumerate()
                {
                    let dx = (*x - *px_old).abs();
                    let dy = (*y - *py_old).abs();

                    // Prevent interpolation across wrap-around
                    let (interp_x, interp_y) =
                        if dx > GRID_WIDTH / 2
                            || dy > GRID_HEIGHT / 2
                        {
                            (*x as f32, *y as f32)
                        } else {
                            (
                                *px_old as f32
                                    + (*x - *px_old) as f32 * t,
                                *py_old as f32
                                    + (*y - *py_old) as f32 * t,
                            )
                        };

                    let (px, py) =
                        grid.to_screen(interp_x, interp_y);

                    let color =
                        if i == 0 { head_color } else { body_color };

                    draw_rectangle(
                        px + 2.0,
                        py + 2.0,
                        grid.cell_size - 4.0,
                        grid.cell_size - 4.0,
                        color,
                    );
                }

                // --- Draw food ---
                let (fx, fy) =
                    grid.to_screen(food.0 as f32, food.1 as f32);

                for particle in &particles {
                    particle.draw();
                }

                let pulse =
                    (get_time().sin() * 1.5 + 1.5) as f32;

                draw_rectangle(
                    fx + 2.0 - pulse,
                    fy + 2.0 - pulse,
                    grid.cell_size - 4.0 + pulse * 2.0,
                    grid.cell_size - 4.0 + pulse * 2.0,
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

                // --- FPS Counter ---
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

            GameState::GameOver => {
                ui::draw_game_over(score);  

                if is_key_pressed(KeyCode::R) {
                    let (s, f, d, t, sc) = reset_game();

                    snake = s;
                    food = f;
                    next_direction = d;
                    move_timer = t;
                    score = sc;

                    prev_body = snake.body.clone();

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