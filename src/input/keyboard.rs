use crate::game::Game;
use crossterm::event::{self, KeyCode};
use std::io;
use std::time::Duration;

pub fn handle_input(game: &mut Game) -> Result<bool, io::Error> {
    if event::poll(Duration::from_millis(100))? {
        if let event::Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Left => game.move_player(-1, 0),
                KeyCode::Right => game.move_player(1, 0),
                KeyCode::Up => game.move_player(0, -1),
                KeyCode::Down => game.move_player(0, 1),
                KeyCode::Char('q') => return Ok(false),
                _ => {}
            }
        }
    }
    Ok(true)
}
