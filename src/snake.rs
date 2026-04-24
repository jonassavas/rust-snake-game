#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Snake {
    pub x: i32,
    pub y: i32,
    pub direction: Direction,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            direction: Direction::Right,
        }
    }

    pub fn update_direction(&mut self, next: Direction) {
        // Prevent reversing
        match (self.direction, next) {
            (Direction::Up, Direction::Down)
            | (Direction::Down, Direction::Up)
            | (Direction::Left, Direction::Right)
            | (Direction::Right, Direction::Left) => {}
            _ => self.direction = next,
        }
    }

    pub fn step(&mut self) {
        match self.direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
}