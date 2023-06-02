use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct Circle {
    pub radius: f32,
    pub position: Vec2,
    pub velocity: Vec2,
}

impl Circle {
    pub fn new(radius: f32, position: Vec2, velocity: Vec2) -> Circle {
        Self { radius, position, velocity }
    }

    pub fn update(&mut self) {
        self.position += self.velocity * get_frame_time();

        let off_left = self.position.x < self.radius;
        let off_right = self.position.x + self.radius > screen_width();

        if off_left && self.velocity.x < 0.0 || off_right && self.velocity.x > 0.0 {
            self.velocity.x = -self.velocity.x;
        }

        let off_top = self.position.y < self.radius;
        let off_bottom = self.position.y + self.radius > screen_height();

        if off_top && self.velocity.y < 0.0 || off_bottom && self.velocity.y > 0.0 {
            self.velocity.y = -self.velocity.y;
        }
    }
}
