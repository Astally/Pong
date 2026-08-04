use macroquad::{prelude::*, rand::gen_range};

const WINDOW_W: f32 = 800.0;
const WINDOW_H: f32 = 600.0;
const PADDLE_W: f32 = 12.0;
const PADDLE_H: f32 = 80.0;
const SHRINK_AMOUNT: f32 = 5.0;
const MIN_PADDLE_H: f32 = 60.0;
const BALL_SIZE: f32 = 12.0;
const PADDLE_OFFSET: f32 = 20.0;
const PADDLE_SPEED: f32 = 400.0; // pixels per second
const WIN_SCORE: u32 = 5;

struct Paddle<'a> {
    rect: Rect,
    texture: &'a Texture2D,
}

impl<'a> Paddle<'a> {
    fn new(x: f32, texture: &'a Texture2D) -> Self {
        Self {
            rect: Rect::new(x, WINDOW_H / 2.0 - PADDLE_H / 2.0, PADDLE_W, PADDLE_H),
            texture,
        }
    }

    fn draw(&self) {
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

    fn update(&mut self, dt: f32, going_up_key: KeyCode, going_down_key: KeyCode) {
        if is_key_down(going_down_key) {
            self.rect.y += PADDLE_SPEED * dt;
        }

        if is_key_down(going_up_key) {
            self.rect.y -= PADDLE_SPEED * dt;
        }

        self.rect.y = clamp(self.rect.y, 0.0, WINDOW_H - self.rect.h);
    }

    fn update_ai(&mut self, ball: &Ball, dt: f32, difficulty: &Difficulty) {
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

    fn shrink(&mut self) {
        self.rect.h = (self.rect.h - SHRINK_AMOUNT).max(MIN_PADDLE_H);

        self.rect.y = clamp(self.rect.y, 0.0, WINDOW_H - self.rect.h);
    }
}

struct Ball<'a> {
    rect: Rect,
    initial_vel: Vec2,
    vel: Vec2,
    speed_level: f32,
    texture: &'a Texture2D,
}

impl<'a> Ball<'a> {
    fn new(texture: &'a Texture2D) -> Self {
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

    fn draw(&self) {
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

    fn update(&mut self, dt: f32) {
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

    fn check_paddles(&mut self, left: &Paddle, right: &Paddle) {
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

    fn increase_speed(&mut self) {
        self.speed_level += 0.1;

        self.speed_level = self.speed_level.min(2.0);
    }

    fn reset(&mut self, point: Point) {
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

    fn reset_game(&mut self) {
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

fn window_conf() -> Conf {
    Conf {
        window_title: "Pong".to_owned(),
        ..Conf::default()
    }
}

fn draw_centre_line() {
    let mut y = 10.0;
    while y < WINDOW_H {
        draw_line(WINDOW_W / 2.0, y, WINDOW_W / 2.0, y + 15.0, 2.0, DARKGRAY);
        y += 25.0;
    }
}

#[derive(Default)]
struct Score {
    left: u32,
    right: u32,
}

enum GameState {
    Menu { selected: usize },
    DifficultyMenu { selected: usize },
    Controls,
    CountDown { timer: f32 },
    Playing,
    GameOver,
}

#[derive(PartialEq)]
enum GameMode {
    SinglePlayer,
    TwoPlayer,
}

enum Difficulty {
    Easy,
    Medium,
    Hard,
}

enum Point {
    Left,
    Right,
}

impl Score {
    fn draw(&self) {
        let text = format!("{}   {}", self.left, self.right);
        let dims = measure_text(&text, None, 48, 1.0);
        draw_text(&text, WINDOW_W / 2.0 - dims.width / 2.0, 48.0, 48.0, WHITE);
    }

    fn update(&mut self, ball: &Ball) -> Option<Point> {
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

struct Game<'a> {
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
async fn main() {
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
