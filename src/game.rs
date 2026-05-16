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
            self.prev_body
                .push(*self.prev_body.last().unwrap());
        }

        let base_delay = 0.12;

        let move_delay =
            (base_delay - self.score as f32 * 0.005).max(0.04);

        let t = (self.move_timer / move_delay).min(1.0);

        for (i, ((x, y), (px_old, py_old))) in self
            .snake
            .body
            .iter()
            .zip(self.prev_body.iter())
            .enumerate()
        {
            let dx = (*x - *px_old).abs();
            let dy = (*y - *py_old).abs();

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
                if i == 0 { self.head_color } else { self.body_color };

            draw_rectangle(
                px + 2.0,
                py + 2.0,
                grid.cell_size - 4.0,
                grid.cell_size - 4.0,
                color,
            );
        }

        let (fx, fy) =
            grid.to_screen(self.food.0 as f32, self.food.1 as f32);

        for particle in &self.particles {
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

        ui::draw_score(self.score);

        ui::draw_fps();
    }
}