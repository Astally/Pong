use crate::Ball;
use crate::Difficulty;
use crate::WINDOW_H;
use crate::constants::{MIN_PADDLE_H, PADDLE_H, PADDLE_SPEED, PADDLE_W, SHRINK_AMOUNT};
use macroquad::prelude::*;

pub struct Paddle<'a> {
    pub rect: Rect,
    pub texture: &'a Texture2D,
}

impl<'a> Paddle<'a> {
    pub fn new(x: f32, texture: &'a Texture2D) -> Self {
        Self {
            rect: Rect::new(x, WINDOW_H / 2.0 - PADDLE_H / 2.0, PADDLE_W, PADDLE_H),
            texture,
        }
    }

    pub fn draw(&self) {
        draw_texture_ex(
            self.texture,
            self.rect.x,
            self.rect.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(self.rect.w, self.rect.h)),
                ..Default::default()
            },
        );
    }

    pub fn update(&mut self, dt: f32, going_up_key: KeyCode, going_down_key: KeyCode) {
        if is_key_down(going_down_key) {
            self.rect.y += PADDLE_SPEED * dt;
        }

        if is_key_down(going_up_key) {
            self.rect.y -= PADDLE_SPEED * dt;
        }

        self.rect.y = clamp(self.rect.y, 0.0, WINDOW_H - self.rect.h);
    }

    pub fn update_ai(&mut self, ball: &Ball, dt: f32, difficulty: &Difficulty) {
        let ball_center = ball.rect.y + ball.rect.h / 2.0;
        let paddle_center = self.rect.y + self.rect.h / 2.0;
        let diff = ball_center - paddle_center;

        if ball.vel.x < 0.0 {
            let (speed, threshold) = match difficulty {
                Difficulty::Easy => (PADDLE_SPEED * 0.55, 25.0),
                Difficulty::Medium => (PADDLE_SPEED * 0.80, 10.0),
                Difficulty::Hard => (PADDLE_SPEED * 0.95, 5.0),
            };

            if diff > threshold {
                self.rect.y += speed * dt;
            } else if diff < -threshold {
                self.rect.y -= speed * dt;
            }

            self.rect.y = clamp(self.rect.y, 0.0, WINDOW_H - self.rect.h);
        }
    }

    pub fn shrink(&mut self) {
        self.rect.h = (self.rect.h - SHRINK_AMOUNT).max(MIN_PADDLE_H);

        self.rect.y = clamp(self.rect.y, 0.0, WINDOW_H - self.rect.h);
    }
}
