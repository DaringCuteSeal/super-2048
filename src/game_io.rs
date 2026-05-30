use crossterm::event::{Event, KeyCode, read};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
};
use std::io::{Write, stdout};
use std::process::exit;

use crate::game_data::GameData;

pub enum Move {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

pub fn get_input() -> Option<Move> {
    enable_raw_mode().unwrap();
    let event = read();
    let mut result: Option<Move> = None;
    if let Err(_) = event {
        return None;
    } else if let Ok(Event::Key(key_event)) = event {
        match key_event.code {
            KeyCode::Char('h') => result = Some(Move::LEFT),
            KeyCode::Char('k') => result = Some(Move::UP),
            KeyCode::Char('j') => result = Some(Move::DOWN),
            KeyCode::Char('l') => result = Some(Move::RIGHT),
            KeyCode::Char('q') => {
                disable_raw_mode().unwrap();
                exit(0)
            }
            _ => result = None,
        }
    }
    disable_raw_mode().unwrap();
    return result;
}

pub fn render(game: &GameData) {
    let mut out = stdout();
    execute!(out, Clear(ClearType::All), MoveTo(0, 0)).unwrap();
    println!("{}", game);
    out.flush().unwrap();
}
