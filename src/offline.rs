use crossterm::{execute, style::*, terminal::*};
use std::io::stdout;

use crate::game;

pub fn offline_player() {
    let mut game = game::Game::new(5);
    execute!(
        stdout(),
        Clear(ClearType::All),
        crossterm::cursor::MoveTo(0, 0),
        SetForegroundColor(Color::Yellow),
        Print(&game.board_string),
        SetForegroundColor(Color::Reset),
    )
    .unwrap();

    let mut turn = true;
    let mut pm = game::CROSS;
    let mut pos = game::Position::new(0, 0);

    while let Some(new_pos) = game.make_move(pos) {
        pos = new_pos;

        game.board[pos.x][pos.y] = pm.move_num;
        execute!(
            stdout(),
            crossterm::cursor::MoveTo(pos.x as u16 * 6 + 3, pos.y as u16 * 2 + 1),
            SetForegroundColor(pm.move_color),
            Print(pm.move_char),
            SetForegroundColor(Color::Reset),
        )
        .unwrap();

        // Switching player
        // bool is wacky - change it later on
        turn = !turn;
        match turn {
            true => pm = game::CROSS,
            false => pm = game::CIRCLE,
        }
    }

    execute!(
        stdout(),
        crossterm::cursor::MoveTo(0, game.board_size as u16 * 2 + 2)
    )
    .unwrap();
}

pub fn offline_minmax() {}
pub fn offline_neuralnetwork() {}
