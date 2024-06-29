mod game;
mod input;
mod rendering;

use game::Game;
use input::handle_input;
use rendering::{cleanup_terminal, draw_game, setup_terminal};
use std::io::Error;
use std::result::Result;

fn main() -> Result<(), Error> {
    setup_terminal()?;

    let mut game = Game::new();

    loop {
        draw_game(&game)?;
        if !handle_input(&mut game)? {
            break;
        }
    }

    cleanup_terminal()?;
    Ok(())
}
