use crate::{Point, ball::Ball, constants::*};
use macroquad::prelude::*;

#[derive(Default)]
pub struct Score {
    pub left: u32,
    pub right: u32,
}

impl Score {
    pub fn draw(&self) {
        let text = format!("{}   {}", self.left, self.right);
        let dims = measure_text(&text, None, 48, 1.0);
        draw_text(&text, WINDOW_W / 2.0 - dims.width / 2.0, 48.0, 48.0, WHITE);
    }

    pub fn update(&mut self, ball: &Ball) -> Option<Point> {
        let left_exit = ball.rect.x + ball.rect.w < 0.0;
        let right_exit = ball.rect.x > WINDOW_W;

        if left_exit {
            self.right += 1;
            Some(Point::Right)
        } else if right_exit {
            self.left += 1;
            Some(Point::Left)
        } else {
            None
        }
    }
}
