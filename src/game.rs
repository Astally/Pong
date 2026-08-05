use crate::{ball::Ball, constants::*, game_draw, paddle::Paddle, score::Score};
use macroquad::prelude::*;

pub enum GameState {
    Menu { selected: usize },
    DifficultyMenu { selected: usize },
    Controls,
    CountDown { timer: f32 },
    Playing,
    Pause { selected: usize },
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

pub enum Point {
    Left,
    Right,
}

pub struct Game<'a> {
    pub ball: Ball<'a>,
    pub left: Paddle<'a>,
    pub right: Paddle<'a>,
    pub score: Score,
    pub game_state: GameState,
    pub game_mode: GameMode,
    pub difficulty: Difficulty,
    pub winner: String,
}

impl<'a> Game<'a> {
    pub fn new(paddle_texture: &'a Texture2D, ball_texture: &'a Texture2D) -> Self {
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

    pub fn update(&mut self, dt: f32, paddle_texture: &'a Texture2D) {
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
                if is_key_pressed(KeyCode::Escape) {
                    self.game_state = GameState::Pause { selected: 0 };
                    return;
                }

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
                }
            }

            GameState::Pause { selected } => {
                if is_key_pressed(KeyCode::Up) {
                    *selected = (*selected + 1) % 2;
                }
                if is_key_pressed(KeyCode::Down) {
                    *selected = (*selected + 1) % 2;
                }

                if is_key_pressed(KeyCode::Enter) {
                    match selected {
                        0 => {
                            self.reset_match(paddle_texture);
                            self.game_state = GameState::Menu { selected: 0 };
                        }
                        1 => {
                            self.game_state = GameState::Playing;
                        }
                        _ => {}
                    }
                }

                if is_key_pressed(KeyCode::Escape) {
                    self.game_state = GameState::Playing;
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

    pub fn draw(&self) {
        game_draw::draw(self);
    }
}
