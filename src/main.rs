use crate::game::Game;
use ball::Ball;
use constants::WINDOW_H;
use macroquad::prelude::*;

mod ball;
mod constants;
mod game;
mod game_draw;
mod paddle;
mod score;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Pong".to_owned(),
        ..Conf::default()
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
