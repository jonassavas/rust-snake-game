use macroquad::prelude::*;

pub const GRID_WIDTH: i32 = 20;
pub const GRID_HEIGHT: i32 = 20;

pub struct Grid {
    pub cell_size: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub width: f32,
    pub height: f32,
}

impl Grid {
    pub fn compute() -> Self {
        let cell_width = screen_width() / GRID_WIDTH as f32;
        let cell_height = screen_height() / GRID_HEIGHT as f32;

        let cell_size = cell_width.min(cell_height);

        let width = cell_size * GRID_WIDTH as f32;
        let height = cell_size * GRID_HEIGHT as f32;

        let offset_x = (screen_width() - width) / 2.0;
        let offset_y = (screen_height() - height) / 2.0;

        Self {
            cell_size,
            offset_x,
            offset_y,
            width,
            height,
        }
    }

    pub fn draw(&self) {
        for x in 0..GRID_WIDTH {
            for y in 0..GRID_HEIGHT {
                let px = self.offset_x + x as f32 * self.cell_size;
                let py = self.offset_y + y as f32 * self.cell_size;

                draw_rectangle_lines(px, py, self.cell_size, self.cell_size, 1.0, Color::from_rgba(60, 60, 60, 255));  
            }
        }

        // Outer border
        draw_rectangle_lines(
            self.offset_x,
            self.offset_y,
            self.width,
            self.height,
            2.0,
            WHITE,
        );
    }

    pub fn to_screen(&self, x: f32, y: f32) -> (f32, f32) {
        (
            self.offset_x + x * self.cell_size,
            self.offset_y + y * self.cell_size,
        )
    }
}