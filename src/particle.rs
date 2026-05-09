use macroquad::prelude::*;

pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub life: f32,
}

impl Particle {
    pub fn new(x: f32, y: f32, vx: f32, vy: f32) -> Self {
        Self {
            x,
            y,
            vx,
            vy,
            life: 0.8,
        } 
    }

    pub fn update(&mut self, dt: f32) {
        self.x += self.vx * dt;
        self.y += self.vy * dt;

        // Slow down slightly
        self.vx *= 0.97;
        self.vy *= 0.97;

        self.life -= dt * 1.1;
    }

    pub fn draw(&self) {
        let color = Color::new(
            1.0,
            0.2,
            0.2,
            self.life.max(0.0),
        );

        draw_circle(self.x, self.y, 4.5, color); 
    }

    pub fn is_dead(&self) -> bool {
        self.life <= 0.0
    }
}