// spring/ elastic froce
use macroquad::prelude::*;
pub struct Ball {
    ball: Circle,
    rest_length: f32,
    velocity: f32,
}

impl Ball {
    pub fn new() -> Self {
        Self {
            ball: Circle {
                x: screen_width() / 2_f32,
                y: 200_f32,
                r: 30_f32,
            },
            rest_length: 400_f32,
            velocity: 0_f32,
        }
    }

    pub fn get_force(&mut self) -> f32 {
        let a = self.ball.y - self.rest_length;
        let k = 0.01_f32;
        -k * a
    }

    pub fn update(&mut self) {
        let force = self.get_force();
        self.velocity += force;
        self.ball.y += self.velocity;
        self.velocity *= 0.99;
    }

    pub fn draw(&mut self) {
        draw_circle(self.ball.x, self.ball.y, self.ball.r, WHITE);
    }
}
