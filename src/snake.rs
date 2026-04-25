#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Snake {
    pub body: Vec<(i32, i32)>,
    pub direction: Direction,
    grow: bool,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            body: vec![(x, y)],
            direction: Direction::Right,
            grow: false,
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
        let (head_x, head_y) = self.body[0];

        let new_head = match self.direction {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        };

        // Add new head at front
        self.body.insert(0, new_head);

        // Remove tail unless growing
        if !self.grow {
            self.body.pop();
        } else {
            self.grow = false;
        }
    }

    pub fn grow(&mut self) {
        self.grow = true;
    }

    pub fn head_position(&self) -> (i32, i32) {
        self.body[0]
    }
}