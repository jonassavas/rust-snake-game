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
            body: vec![
                (x, y),
                (x - 1, y),
                (x - 2, y),
            ], 
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

    pub fn step(&mut self, grid_width: i32, grid_height: i32) -> bool {
    let (head_x, head_y) = self.body[0];

    let mut new_head = match self.direction {
        Direction::Up => (head_x, head_y - 1),
        Direction::Down => (head_x, head_y + 1),
        Direction::Left => (head_x - 1, head_y),
        Direction::Right => (head_x + 1, head_y),
    };

    // --- Wrap around ---
    if new_head.0 < 0 {
        new_head.0 = grid_width - 1;
    }
    if new_head.0 >= grid_width {
        new_head.0 = 0;
    }
    if new_head.1 < 0 {
        new_head.1 = grid_height - 1;
    }
    if new_head.1 >= grid_height {
        new_head.1 = 0;
    }

    // --- Check collision BEFORE modifying body ---
    if self.body.contains(&new_head) {
        return true; // collision detected
    }

    // Insert new head
    self.body.insert(0, new_head);

    // Remove tail unless growing
    if !self.grow {
        self.body.pop();
    } else {
        self.grow = false;
    }

    false
} 

    pub fn check_self_collision(&self) -> bool {
        let head = self.body[0];

        // Check if head overlaps any body segment
        self.body.iter().skip(1).any(|&segment| segment == head)
    }

    pub fn grow(&mut self) {
        self.grow = true;
    }

    pub fn head_position(&self) -> (i32, i32) {
        self.body[0]
    }
}