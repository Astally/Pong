use macroquad::{prelude::*, rand::gen_range};

use crate::{Point, constants::*, paddle::Paddle};

pub struct Ball<'a> {
    pub rect: Rect,
    pub initial_vel: Vec2,
    pub vel: Vec2,
    pub speed_level: f32,
    pub texture: &'a Texture2D,
}

impl<'a> Ball<'a> {
    pub fn new(texture: &'a Texture2D) -> Self {
        Self {
            rect: Rect::new(
                WINDOW_W / 2.0 - BALL_SIZE / 2.0,
                WINDOW_H / 2.0 - BALL_SIZE / 2.0,
                BALL_SIZE,
                BALL_SIZE,
            ),
            initial_vel: Vec2::new(300.0, 220.0),
            vel: Vec2::new(300.0, 220.0),
            speed_level: 1.0,
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

    pub fn update(&mut self, dt: f32) {
        self.rect.x += self.vel.x * dt;
        self.rect.y += self.vel.y * dt;

        // bounce off top wall
        if self.rect.y < 0.0 {
            self.rect.y = 0.0;
            self.vel.y = self.vel.y.abs();
        }
        // bounce off bottom wall
        if self.rect.y + self.rect.h > WINDOW_H {
            self.rect.y = WINDOW_H - self.rect.h;
            self.vel.y = -self.vel.y.abs();
        }
    }

    pub fn check_paddles(&mut self, left: &Paddle, right: &Paddle) {
        if self.rect.overlaps(&left.rect) {
            self.rect.x = left.rect.x + left.rect.w; // push ball out
            self.calculate_bounce_angle(left, true);
        }

        if self.rect.overlaps(&right.rect) {
            self.rect.x = right.rect.x - self.rect.w; // push ball out
            self.calculate_bounce_angle(right, false);
        }
    }

    fn calculate_bounce_angle(&mut self, paddle: &Paddle, is_left: bool) {
        let ball_center = self.rect.y + self.rect.h / 2.0;
        let paddle_center = paddle.rect.y + paddle.rect.h / 2.0;

        let offset = ball_center - paddle_center;

        let normalized = offset / (paddle.rect.h / 2.0);
        let normalized = normalized.clamp(-1.0, 1.0);

        let angle = (normalized * 60.0).to_radians();

        let speed = self.vel.length();

        if is_left {
            self.vel.x = speed * angle.cos();
        } else {
            self.vel.x = -speed * angle.cos();
        }

        self.vel.y = speed * angle.sin();
    }

    pub fn increase_speed(&mut self) {
        self.speed_level += 0.1;

        self.speed_level = self.speed_level.min(2.0);
    }

    pub fn reset(&mut self, point: Point) {
        self.rect.x = WINDOW_W / 2.0 - BALL_SIZE / 2.0;
        self.rect.y = WINDOW_H / 2.0 - BALL_SIZE / 2.0;

        let angle: f32 = gen_range(-40.0, 40.0);
        let angle = angle.to_radians();

        let speed = self.initial_vel.length() * self.speed_level;

        self.vel.y = speed * angle.sin();

        match point {
            Point::Left => {
                self.vel.x = -speed * angle.cos();
            }
            Point::Right => {
                self.vel.x = speed * angle.cos();
            }
        }
    }

    pub fn reset_game(&mut self) {
        self.rect.x = WINDOW_W / 2.0 - BALL_SIZE / 2.0;
        self.rect.y = WINDOW_H / 2.0 - BALL_SIZE / 2.0;

        self.speed_level = 1.0;

        if gen_range(0, 2) == 0 {
            self.reset(Point::Left);
        } else {
            self.reset(Point::Right);
        }
    }
}
