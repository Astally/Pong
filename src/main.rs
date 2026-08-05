use crate::game::Game;
use ball::Ball;
use constants::{WINDOW_H, WINDOW_W};
use macroquad::prelude::*;

mod ball;
mod constants;
mod game;
mod paddle;
mod score;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Pong".to_owned(),
        ..Conf::default()
    }
}

pub fn draw_centre_line() {
    let mut y = 10.0;
    while y < WINDOW_H {
        draw_line(WINDOW_W / 2.0, y, WINDOW_W / 2.0, y + 15.0, 2.0, DARKGRAY);
        y += 25.0;
    }
}

#[macroquad::main(window_conf)]
pub async fn main() {
    let ball_texture = load_texture("assets/ball.png").await.unwrap();
    let paddle_texture = load_texture("assets/paddle.png").await.unwrap();

    let mut game = Game::new(&paddle_texture, &ball_texture);

    loop {
        let dt = get_frame_time();

        game.update(dt, &paddle_texture);

        game.draw();

        next_frame().await;
    }
}
