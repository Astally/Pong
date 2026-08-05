use crate::score::Score;
use ball::Ball;
use macroquad::prelude::*;
use paddle::Paddle;

use constants::{PADDLE_OFFSET, PADDLE_W, WIN_SCORE, WINDOW_H, WINDOW_W};

mod ball;
mod constants;
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

pub enum GameState {
    Menu { selected: usize },
    DifficultyMenu { selected: usize },
    Controls,
    CountDown { timer: f32 },
    Playing,
    GameOver,
}

#[derive(PartialEq)]
pub enum GameMode {
    SinglePlayer,
    TwoPlayer,
}

pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

enum Point {
    Left,
    Right,
}

pub struct Game<'a> {
    ball: Ball<'a>,
    left: Paddle<'a>,
    right: Paddle<'a>,
    score: Score,
    game_state: GameState,
    game_mode: GameMode,
    difficulty: Difficulty,
    winner: String,
}

impl<'a> Game<'a> {
    fn new(paddle_texture: &'a Texture2D, ball_texture: &'a Texture2D) -> Self {
        let game_state = GameState::Menu { selected: 0 };
        let game_mode = GameMode::SinglePlayer;

        let score = Score::default();
        let winner = "".to_string();
        let ball = Ball::new(ball_texture);
        let left = Paddle::new(PADDLE_OFFSET, paddle_texture);
        let right = Paddle::new(WINDOW_W - PADDLE_W - PADDLE_OFFSET, paddle_texture);

        Self {
            ball,
            left,
            right,
            score,
            game_state,
            game_mode,
            difficulty: Difficulty::Medium,
            winner,
        }
    }

    fn update(&mut self, dt: f32, paddle_texture: &'a Texture2D) {
        match &mut self.game_state {
            GameState::Menu { selected } => {
                if is_key_pressed(KeyCode::Down) && *selected < 3 {
                    *selected += 1;
                }

                if is_key_pressed(KeyCode::Up) {
                    *selected = (*selected).saturating_sub(1);
                }

                if is_key_pressed(KeyCode::Enter) {
                    self.game_state = match selected {
                        0 => {
                            self.reset_match(paddle_texture);

                            self.game_mode = GameMode::SinglePlayer;

                            GameState::DifficultyMenu { selected: 0 }
                        }
                        1 => {
                            self.reset_match(paddle_texture);

                            self.game_mode = GameMode::TwoPlayer;

                            GameState::CountDown { timer: 4.0 }
                        }
                        2 => GameState::Controls,
                        3 => std::process::exit(0),
                        _ => GameState::Menu { selected: 0 },
                    }
                }
            }

            GameState::DifficultyMenu { selected } => {
                if is_key_pressed(KeyCode::Down) && *selected < 3 {
                    *selected += 1;
                }
                if is_key_pressed(KeyCode::Up) {
                    *selected = (*selected).saturating_sub(1);
                }

                if is_key_pressed(KeyCode::Enter) {
                    match selected {
                        0 => self.difficulty = Difficulty::Easy,
                        1 => self.difficulty = Difficulty::Medium,
                        2 => self.difficulty = Difficulty::Hard,
                        3 => {
                            self.game_state = GameState::Menu { selected: 0 };
                            return;
                        }
                        _ => {}
                    }

                    self.reset_match(paddle_texture);
                    self.game_state = GameState::CountDown { timer: 4.0 };
                }

                if is_key_pressed(KeyCode::Escape) {
                    self.game_state = GameState::Menu { selected: 0 };
                }
            }

            GameState::Controls => {
                if is_key_pressed(KeyCode::Escape) {
                    self.game_state = GameState::Menu { selected: 0 };
                }
            }

            GameState::CountDown { timer } => {
                self.right.update(dt, KeyCode::Up, KeyCode::Down);

                match self.game_mode {
                    GameMode::SinglePlayer => {
                        self.left.update_ai(&self.ball, dt, &self.difficulty);
                    }

                    GameMode::TwoPlayer => {
                        self.left.update(dt, KeyCode::W, KeyCode::S);
                    }
                }

                *timer -= dt;

                if *timer <= 0.0 {
                    self.game_state = GameState::Playing;
                }
            }

            GameState::Playing => {
                self.right.update(dt, KeyCode::Up, KeyCode::Down);

                match self.game_mode {
                    GameMode::SinglePlayer => {
                        self.left.update_ai(&self.ball, dt, &self.difficulty);
                    }

                    GameMode::TwoPlayer => {
                        self.left.update(dt, KeyCode::W, KeyCode::S);
                    }
                }

                self.ball.update(dt);
                self.ball.check_paddles(&self.left, &self.right);

                if let Some(point) = self.score.update(&self.ball) {
                    if self.score.left >= WIN_SCORE {
                        self.winner = "Left player wins!".to_string();
                        self.game_state = GameState::GameOver;
                        return;
                    }

                    if self.score.right >= WIN_SCORE {
                        self.winner = "Right player wins!".to_string();
                        self.game_state = GameState::GameOver;
                        return;
                    }

                    match point {
                        Point::Right => {
                            self.left.shrink();
                        }
                        Point::Left => {
                            self.right.shrink();
                        }
                    }

                    self.ball.increase_speed();
                    self.ball.reset(point);

                    self.game_state = GameState::CountDown { timer: 4.0 };
                    return;
                }
            }

            GameState::GameOver => {
                if is_key_pressed(KeyCode::R) {
                    self.reset_match(paddle_texture);

                    self.game_state = GameState::CountDown { timer: 4.0 };
                }

                if is_key_pressed(KeyCode::Escape) {
                    self.reset_match(paddle_texture);

                    self.game_state = GameState::Menu { selected: 0 };
                }
            }
        }
    }

    fn reset_match(&mut self, paddle_texture: &'a Texture2D) {
        self.score = Score::default();
        self.ball.reset_game();

        self.left = Paddle::new(PADDLE_OFFSET, paddle_texture);
        self.right = Paddle::new(WINDOW_W - PADDLE_OFFSET - PADDLE_W, paddle_texture);
    }

    fn draw(&self) {
        match self.game_state {
            GameState::Menu { selected } => {
                clear_background(BLACK);

                let dims = measure_text("Pong", None, 50, 1.0);
                draw_text(
                    "Pong",
                    WINDOW_W / 2.0 - dims.width / 2.0,
                    WINDOW_H * 1.0 / 7.0,
                    50.0,
                    SKYBLUE,
                );

                const MENU_ITEMS: [&str; 4] = ["Single Player", "Two Player", "Controls", "Exit"];

                for (index, items) in MENU_ITEMS.iter().enumerate() {
                    let counter = index as f32 + 2.0;

                    let dims = measure_text(items, None, 30, 1.0);
                    draw_text(
                        items,
                        WINDOW_W / 2.0 - dims.width / 2.0,
                        WINDOW_H * counter / 7.0,
                        30.0,
                        if index == selected { YELLOW } else { GREEN },
                    );
                }
            }

            GameState::Controls => {
                clear_background(BLACK);

                let dims = measure_text("CONTROLS", None, 50, 1.0);
                draw_text(
                    "CONTROLS",
                    WINDOW_W / 2.0 - dims.width / 2.0,
                    WINDOW_H * 1.0 / 12.0,
                    50.0,
                    SKYBLUE,
                );

                const PLAYER_ITEMS: [&str; 2] = [("Left Player"), ("Right Player")];

                for (index, items) in PLAYER_ITEMS.iter().enumerate() {
                    let counter = index as f32 + 1.0;

                    draw_text(
                        items,
                        WINDOW_W * 0.15,
                        WINDOW_H * counter / 5.0,
                        32.0,
                        YELLOW,
                    );
                }

                const LEFT_PLAYER_ITEMS: [(&str, &str); 2] = [("W", "Move Up"), ("S", "Move Down")];

                for (index, (key, action)) in LEFT_PLAYER_ITEMS.iter().enumerate() {
                    let counter = index as f32 + 5.0;

                    draw_text(
                        key,
                        WINDOW_W * 0.15,
                        WINDOW_H * counter / 20.0,
                        24.0,
                        ORANGE,
                    );

                    draw_text(
                        action,
                        WINDOW_W * 0.35,
                        WINDOW_H * counter / 20.0,
                        24.0,
                        WHITE,
                    );
                }

                const RIGHT_PLAYER_ITEMS: [(&str, &str); 2] =
                    [("Up Arrow", "Move Up"), ("Down Arrow", "Move Down")];

                for (index, (key, action)) in RIGHT_PLAYER_ITEMS.iter().enumerate() {
                    let counter = index as f32 + 9.0;

                    draw_text(
                        key,
                        WINDOW_W * 0.15,
                        WINDOW_H * counter / 20.0,
                        24.0,
                        ORANGE,
                    );

                    draw_text(
                        action,
                        WINDOW_W * 0.35,
                        WINDOW_H * counter / 20.0,
                        24.0,
                        WHITE,
                    );
                }

                const GENERAL_ITEMS: [(&str, &str); 3] = [
                    ("Enter", "Select"),
                    ("Esc", "Back to Menu"),
                    ("R", "Restart Game"),
                ];

                for (index, (key, action)) in GENERAL_ITEMS.iter().enumerate() {
                    let counter = index as f32 + 13.0;

                    draw_text(
                        key,
                        WINDOW_W * 0.15,
                        WINDOW_H * counter / 20.0,
                        24.0,
                        ORANGE,
                    );

                    draw_text(
                        action,
                        WINDOW_W * 0.35,
                        WINDOW_H * counter / 20.0,
                        24.0,
                        WHITE,
                    );
                }

                draw_text(
                    "Press ESC to return",
                    WINDOW_W * 0.15,
                    WINDOW_H * 18.0 / 20.0,
                    22.0,
                    GRAY,
                );
            }

            GameState::DifficultyMenu { selected } => {
                clear_background(BLACK);

                let dims = measure_text("Select Difficulty", None, 40, 1.0);
                draw_text(
                    "Select Difficulty",
                    WINDOW_W / 2.0 - dims.width / 2.0,
                    WINDOW_H * 1.0 / 7.0,
                    40.0,
                    SKYBLUE,
                );

                const DIFFICULTY_ITEMS: [&str; 4] = ["Easy", "Medium", "Hard", "Back"];

                for (index, items) in DIFFICULTY_ITEMS.iter().enumerate() {
                    let counter = index as f32 + 2.0;

                    let dims = measure_text(items, None, 30, 1.0);
                    draw_text(
                        items,
                        WINDOW_W / 2.0 - dims.width / 2.0,
                        WINDOW_H * counter / 7.0,
                        30.0,
                        if index == selected { YELLOW } else { GREEN },
                    );
                }
            }

            GameState::CountDown { timer } => {
                clear_background(BLACK);
                draw_centre_line();

                self.left.draw();
                self.right.draw();
                self.ball.draw();
                self.score.draw();

                let text = if timer > 1.0 {
                    (timer - 1.0).ceil().to_string()
                } else {
                    "GO!".to_string()
                };

                let dims = measure_text(&text, None, 120, 1.0);

                draw_text(
                    &text,
                    WINDOW_W / 2.0 - dims.width / 2.0,
                    WINDOW_H / 2.0,
                    120.0,
                    WHITE,
                );
            }

            GameState::Playing => {
                clear_background(BLACK);
                draw_centre_line();

                self.left.draw();
                self.right.draw();
                self.ball.draw();
                self.score.draw();
            }

            GameState::GameOver => {
                let dims = measure_text(&self.winner, None, 48, 1.0);
                draw_text(
                    &self.winner,
                    WINDOW_W / 2.0 - dims.width / 2.0,
                    WINDOW_H / 2.0,
                    48.0,
                    WHITE,
                );

                let hint = "Press R to restart";
                let hdims = measure_text(hint, None, 24, 1.0);
                draw_text(
                    hint,
                    WINDOW_W / 2.0 - hdims.width / 2.0,
                    WINDOW_H / 2.0 + 40.0,
                    24.0,
                    GRAY,
                );

                let hint = "Press Esc to back to the Main Menu";
                let hdims = measure_text(hint, None, 24, 1.0);
                draw_text(
                    hint,
                    WINDOW_W / 2.0 - hdims.width / 2.0,
                    WINDOW_H / 2.0 + 70.0,
                    24.0,
                    GRAY,
                );
            }
        }
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
