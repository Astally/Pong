use crate::{constants::*, game::Game, game::GameState};
use macroquad::prelude::*;

pub fn draw_centre_line() {
    let mut y = 10.0;
    while y < WINDOW_H {
        draw_line(WINDOW_W / 2.0, y, WINDOW_W / 2.0, y + 15.0, 2.0, DARKGRAY);
        y += 25.0;
    }
}

pub fn draw(game: &Game) {
    match game.game_state {
        GameState::Menu { selected } => draw_menu(selected),

        GameState::Controls => draw_controls(),

        GameState::DifficultyMenu { selected } => draw_difficulty(selected),

        GameState::CountDown { timer } => draw_countdown(game, timer),

        GameState::Playing => draw_playing(game),

        GameState::Pause { selected } => draw_pause(game, selected),

        GameState::GameOver => draw_game_over(game),
    }
}

fn draw_menu(selected: usize) {
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

fn draw_controls() {
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

fn draw_difficulty(selected: usize) {
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

fn draw_countdown(game: &Game, timer: f32) {
    clear_background(BLACK);
    draw_centre_line();

    game.left.draw();
    game.right.draw();
    game.ball.draw();
    game.score.draw();

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

fn draw_playing(game: &Game) {
    clear_background(BLACK);
    draw_centre_line();

    game.left.draw();
    game.right.draw();
    game.ball.draw();
    game.score.draw();
}

fn draw_game_over(game: &Game) {
    let dims = measure_text(&game.winner, None, 48, 1.0);
    draw_text(
        &game.winner,
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

fn draw_pause(game: &Game, selected: usize) {
    draw_playing(game);

    draw_rectangle(0.0, 0.0, WINDOW_W, WINDOW_H, Color::new(0.0, 0.0, 0.0, 0.7));

    let text = "Quit to Main Menu?";
    let dims = measure_text(text, None, 40, 1.0);
    draw_text(
        text,
        WINDOW_W / 2.0 - dims.width / 2.0,
        WINDOW_H / 2.0 - 60.0,
        40.0,
        WHITE,
    );

    let items = ["Yes", "No"];
    for (index, item) in items.iter().enumerate() {
        let color = if index == selected { YELLOW } else { GREEN };
        let y_pos = WINDOW_H / 2.0 + (index as f32 * 50.0);

        let dims = measure_text(item, None, 30, 1.0);
        draw_text(item, WINDOW_W / 2.0 - dims.width / 2.0, y_pos, 30.0, color);
    }
}
