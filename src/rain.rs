use macroquad::prelude::*;
pub struct Rain {
    rect: Rect,
    vel: Vec2,
}

impl Rain {
    pub fn new() -> Self {
        Self {
            rect: Rect {
                x: rand::gen_range(0_f32, screen_width()),
                y: rand::gen_range(0_f32, screen_height()),
                w: rand::gen_range(5_f32, 10_f32),
                h: rand::gen_range(50_f32, 80_f32),
            },
            vel: Vec2::from_array([0f32, 100f32]),
        }
    }
    pub fn fall(&mut self, dt: f32) {
        self.rect.y += self.vel.y * dt;
        if self.rect.y > screen_height() {
            self.rect.y = -70.0;
        }

        if self.rect.w > 8_f32 && self.rect.h > 70_f32 {
            self.rect.y += self.vel.y * dt * 0.5;
        }
    }
    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, WHITE);
    }
}
