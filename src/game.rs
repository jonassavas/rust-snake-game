use macroquad::prelude::*;

use crate::grid::{Grid, GRID_HEIGHT, GRID_WIDTH};
use crate::particle::Particle;
use crate::snake::{Direction, Snake};
use crate::ui;

#[derive(PartialEq)]
pub enum GameState {
    Menu,
    Playing,
    Paused,
    GameOver,
}

pub struct Game {
    pub state: GameState,

    pub snake: Snake,
    pub food: (i32, i32),

    pub next_direction: Direction,

    pub move_timer: f32,

    pub score: i32,

    pub particles: Vec<Particle>,

    pub prev_body: Vec<(i32, i32)>,

    pub head_color: Color,
    pub body_color: Color,

    pub tongue_timer: f32,
    pub tongue_active: bool,
}

fn draw_rounded_rect(
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    radius: f32,
    color: Color,
) {
    // Center
    draw_rectangle(
        x + radius,
        y,
        w - radius * 2.0,
        h,
        color,
    );

    // Sides
    draw_rectangle(
        x,
        y + radius,
        radius,
        h - radius * 2.0,
        color,
    );

    draw_rectangle(
        x + w - radius,
        y + radius,
        radius,
        h - radius * 2.0,
        color,
    );

    // Corners
    draw_circle(x + radius, y + radius, radius, color);

    draw_circle(
        x + w - radius,
        y + radius,
        radius,
        color,
    );

    draw_circle(
        x + radius,
        y + h - radius,
        radius,
        color,
    );

    draw_circle(
        x + w - radius,
        y + h - radius,
        radius,
        color,
    );
}

impl Game {
    pub fn new() -> Self {
        let snake = Snake::new(10, 10);

        Self {
            state: GameState::Menu,

            food: Self::random_food(&snake),

            next_direction: Direction::Right,

            move_timer: 0.0,

            score: 0,

            prev_body: snake.body.clone(),

            particles: Vec::new(),

            head_color: Color::from_rgba(80, 220, 120, 255),

            body_color: Color::from_rgba(40, 160, 80, 255),

            snake,

            tongue_timer: 0.0,
            tongue_active: false,
        }
    }

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

    pub fn reset(&mut self) {
        self.snake = Snake::new(10, 10);

        self.food = Self::random_food(&self.snake);

        self.next_direction = Direction::Right;

        self.move_timer = 0.0;

        self.score = 0;

        self.prev_body = self.snake.body.clone();

        self.particles.clear();

        self.tongue_timer = 0.0;
        self.tongue_active = false;
    }

    pub fn update(&mut self) {
        match self.state {
            GameState::Menu => {
                if is_key_pressed(KeyCode::Enter) {
                    self.reset();
                    self.state = GameState::Playing;
                }
            }

            GameState::Playing => {
                if is_key_pressed(KeyCode::Escape){
                    self.state = GameState::Paused;
                    return;
                }
                self.handle_input();
                self.update_game();
            }

            GameState::GameOver => {
                if is_key_pressed(KeyCode::R) {
                    self.reset();
                    self.state = GameState::Playing;
                }

                if is_key_pressed(KeyCode::Escape) {
                    self.state = GameState::Menu;
                }
            }

            GameState::Paused => {
                if is_key_pressed(KeyCode::Escape) {
                    self.state = GameState::Playing;
                }

                if is_key_pressed(KeyCode::M) {
                    self.state = GameState::Menu;
                }
            }
        }
    }

    fn handle_input(&mut self) {
        if is_key_pressed(KeyCode::Up) {
            self.next_direction = Direction::Up;
        }

        if is_key_pressed(KeyCode::Down) {
            self.next_direction = Direction::Down;
        }

        if is_key_pressed(KeyCode::Left) {
            self.next_direction = Direction::Left;
        }

        if is_key_pressed(KeyCode::Right) {
            self.next_direction = Direction::Right;
        }
    }

    fn update_game(&mut self) {
        let base_delay = 0.12;

        let move_delay =
            (base_delay - self.score as f32 * 0.005).max(0.04);

        self.move_timer += get_frame_time();

        while self.move_timer >= move_delay {
            self.move_timer -= move_delay;

            self.snake.update_direction(self.next_direction);

            self.prev_body = self.snake.body.clone();

            if self.snake.step(GRID_WIDTH, GRID_HEIGHT) {
                self.state = GameState::GameOver;
                return;
            }

            if self.snake.head_position() == self.food {
                self.snake.grow();

                self.score += 1;

                self.spawn_food_particles();

                self.food = Self::random_food(&self.snake);
            }
        }

        let dt = get_frame_time();

        for particle in self.particles.iter_mut() {
            particle.update(dt);
        }

        self.particles.retain(|p| !p.is_dead());

        self.tongue_timer += get_frame_time();

        if self.tongue_timer > 1.5 {
            self.tongue_timer = 0.0;
            self.tongue_active = true;
        }

        if self.tongue_active {
            self.tongue_timer += get_frame_time();

            if self.tongue_timer > 0.15 {
                self.tongue_active = false;
                self.tongue_timer = 0.0;
            }
        }
    }

    fn spawn_food_particles(&mut self) {
        let grid = Grid::compute();

        let (fx, fy) =
            grid.to_screen(self.food.0 as f32, self.food.1 as f32);

        let center_x = fx + grid.cell_size / 2.0;
        let center_y = fy + grid.cell_size / 2.0;

        let speed = 180.0;
        let offset = 10.0;

        self.particles.push(Particle::new(
            center_x - offset,
            center_y - offset,
            -speed,
            -speed,
        ));

        self.particles.push(Particle::new(
            center_x + offset,
            center_y - offset,
            speed,
            -speed,
        ));

        self.particles.push(Particle::new(
            center_x - offset,
            center_y + offset,
            -speed,
            speed,
        ));

        self.particles.push(Particle::new(
            center_x + offset,
            center_y + offset,
            speed,
            speed,
        ));
    }

    pub fn draw(&mut self) {
        match self.state {
            GameState::Menu => {
                ui::draw_menu();
            }

            GameState::Playing => {
                self.draw_game();
            }

            GameState::GameOver => {
                ui::draw_game_over(self.score);
            }

            GameState::Paused => {
                self.draw_game();

                draw_rectangle(
                    0.0,
                    0.0,
                    screen_width(),
                    screen_height(),
                    Color::new(0.0, 0.0, 0.0, 0.5),
                );

                ui::draw_pause_menu();
            }
        }
    }

    fn draw_game(&mut self) {
        let grid = Grid::compute();
        grid.draw();

        while self.prev_body.len() < self.snake.body.len() {
            self.prev_body.push(*self.prev_body.last().unwrap());
        }

        let base_delay = 0.12;
        let move_delay = (base_delay - self.score as f32 * 0.005).max(0.04);

        let t = (self.move_timer / move_delay).min(1.0);

        let (dx_dir, dy_dir) = match self.snake.direction {
            Direction::Up => (0.0, -1.0),
            Direction::Down => (0.0, 1.0),
            Direction::Left => (-1.0, 0.0),
            Direction::Right => (1.0, 0.0),
        };

        // --- DRAW SNAKE ---
        for (i, ((x, y), (px_old, py_old))) in self
            .snake
            .body
            .iter()
            .zip(self.prev_body.iter())
            .enumerate()
        {
            let dx = (*x - *px_old).abs();
            let dy = (*y - *py_old).abs();

            let (interp_x, interp_y) = if dx > GRID_WIDTH / 2 || dy > GRID_HEIGHT / 2 {
                (*x as f32, *y as f32)
            } else {
                (
                    *px_old as f32 + (*x - *px_old) as f32 * t,
                    *py_old as f32 + (*y - *py_old) as f32 * t,
                )
            };

            let (px, py) = grid.to_screen(interp_x, interp_y);

            let color = if i == 0 {
                self.head_color
            } else {
                self.body_color
            };

            let rect_x = px + 2.0;
            let rect_y = py + 2.0;

            // --- TAPER ---
            let mut size = grid.cell_size - 4.0;

            if i > 0 {
                let tt = i as f32 / self.snake.body.len() as f32;
                size *= 1.0 - tt * 0.35;
            }

            let offset = (grid.cell_size - 4.0 - size) / 2.0;

            let draw_x = rect_x + offset;
            let draw_y = rect_y + offset;

            draw_rounded_rect(
                draw_x,
                draw_y,
                size,
                size,
                size * 0.22,
                color,
            );

            // =========================
            // TONGUE (HEAD ONLY)
            // =========================
            if i == 0 && self.tongue_active {
                let tongue_len = grid.cell_size * 0.6;

                let start_x = px + grid.cell_size / 2.0;
                let start_y = py + grid.cell_size / 2.0;

                let end_x = start_x + dx_dir * tongue_len;
                let end_y = start_y + dy_dir * tongue_len;

                draw_line(
                    start_x,
                    start_y,
                    end_x,
                    end_y,
                    2.0,
                    Color::new(1.0, 0.3, 0.3, 1.0),
                );

                draw_line(
                    end_x,
                    end_y,
                    end_x + dy_dir * 3.0,
                    end_y - dx_dir * 3.0,
                    1.5,
                    RED,
                );

                draw_line(
                    end_x,
                    end_y,
                    end_x - dy_dir * 3.0,
                    end_y + dx_dir * 3.0,
                    1.5,
                    RED,
                );
            }

            // =========================
            // EYES (HEAD ONLY)
            // =========================
            if i == 0 {
                let eye_radius = size * 0.08;

                let (eye1_x, eye1_y, eye2_x, eye2_y) = match self.snake.direction {
                    Direction::Up => (
                        draw_x + size * 0.30,
                        draw_y + size * 0.25,
                        draw_x + size * 0.70,
                        draw_y + size * 0.25,
                    ),
                    Direction::Down => (
                        draw_x + size * 0.30,
                        draw_y + size * 0.75,
                        draw_x + size * 0.70,
                        draw_y + size * 0.75,
                    ),
                    Direction::Left => (
                        draw_x + size * 0.25,
                        draw_y + size * 0.30,
                        draw_x + size * 0.25,
                        draw_y + size * 0.70,
                    ),
                    Direction::Right => (
                        draw_x + size * 0.75,
                        draw_y + size * 0.30,
                        draw_x + size * 0.75,
                        draw_y + size * 0.70,
                    ),
                };

                let blink = (get_time() * 2.5).sin() > 0.97;

                if blink {
                    draw_line(eye1_x - eye_radius, eye1_y, eye1_x + eye_radius, eye1_y, 2.0, BLACK);
                    draw_line(eye2_x - eye_radius, eye2_y, eye2_x + eye_radius, eye2_y, 2.0, BLACK);
                } else {
                    draw_circle(eye1_x, eye1_y, eye_radius * 1.6, WHITE);
                    draw_circle(eye2_x, eye2_y, eye_radius * 1.6, WHITE);

                    draw_circle(eye1_x, eye1_y, eye_radius, BLACK);
                    draw_circle(eye2_x, eye2_y, eye_radius, BLACK);
                }
            }
        }

        // --- FOOD ---
        let (fx, fy) =
            grid.to_screen(self.food.0 as f32, self.food.1 as f32);

        for particle in &self.particles {
            particle.draw();
        }

        let pulse = (get_time().sin() * 1.5 + 1.5) as f32;

        draw_rectangle(
            fx + 2.0 - pulse,
            fy + 2.0 - pulse,
            grid.cell_size - 4.0 + pulse * 2.0,
            grid.cell_size - 4.0 + pulse * 2.0,
            RED,
        );

        ui::draw_score(self.score);
        ui::draw_fps();
    } 

}