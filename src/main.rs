use macroquad::prelude::*;

const GRID_WIDTH: i32 = 20;
const GRID_HEIGHT: i32 = 20;

// Represents movement direction
#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// Snake with position + direction
struct Snake {
    x: i32,
    y: i32,
    direction: Direction,
}

// Attribute macro:
// - Creates window
// - Sets up graphics + game loop
// - Names window "Snake"
#[macroquad::main("Snake")]
async fn main() {
    // Snake starts in the middle moving right
    let mut snake = Snake {
        x: 10,
        y: 10,
        direction: Direction::Right,
    };

    let mut prev_x = snake.x;
    let mut prev_y = snake.y;
    let mut next_direction = snake.direction;

    // Movement timing (seconds)
    let mut move_timer = 0.0;
    let move_delay = 0.2;

    loop {
        clear_background(BLACK);

        // --- Handle input (change direction) ---
        if is_key_pressed(KeyCode::Up) && !matches!(snake.direction, Direction::Down) {
            next_direction = Direction::Up;
        }
        if is_key_pressed(KeyCode::Down) && !matches!(snake.direction, Direction::Up) {
            next_direction = Direction::Down;
        }
        if is_key_pressed(KeyCode::Left) && !matches!(snake.direction, Direction::Right) {
            next_direction = Direction::Left;
        }
        if is_key_pressed(KeyCode::Right) && !matches!(snake.direction, Direction::Left) {
            next_direction = Direction::Right;
        }

        // --- Time-based movement ---
        move_timer += get_frame_time();

        if move_timer >= move_delay {
            move_timer = 0.0;

            snake.direction = next_direction;

            prev_x = snake.x;
            prev_y = snake.y;

            match snake.direction {
                Direction::Up => snake.y -= 1,
                Direction::Down => snake.y += 1,
                Direction::Left => snake.x -= 1,
                Direction::Right => snake.x += 1,
            }
        }

        // --- Compute dynamic grid ---
        let cell_width = screen_width() / GRID_WIDTH as f32;
        let cell_height = screen_height() / GRID_HEIGHT as f32;

        // Keep cells square
        let cell_size = cell_width.min(cell_height);

        let grid_pixel_width = cell_size * GRID_WIDTH as f32;
        let grid_pixel_height = cell_size * GRID_HEIGHT as f32;

        // Center grid
        let offset_x = (screen_width() - grid_pixel_width) / 2.0;
        let offset_y = (screen_height() - grid_pixel_height) / 2.0;

        // --- Draw grid ---
        for x in 0..GRID_WIDTH {
            for y in 0..GRID_HEIGHT {
                let px = offset_x + x as f32 * cell_size;
                let py = offset_y + y as f32 * cell_size;

                draw_rectangle_lines(px, py, cell_size, cell_size, 1.0, DARKGRAY);
            }
        }

        let t = (move_timer / move_delay).min(1.0); // 0.0 → 1.0

        let interp_x = prev_x as f32 + (snake.x - prev_x) as f32 * t;
        let interp_y = prev_y as f32 + (snake.y - prev_y) as f32 * t;
        // --- Draw snake ---
        let px = offset_x + interp_x * cell_size;
        let py = offset_y + interp_y * cell_size;
        draw_rectangle(px, py, cell_size, cell_size, GREEN);

        // --- Outer border ---
        draw_rectangle_lines(
            offset_x,
            offset_y,
            grid_pixel_width,
            grid_pixel_height,
            2.0,
            WHITE,
        );

        // Yield to engine (render + wait for next frame)
        next_frame().await;
    }
}

