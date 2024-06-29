use crate::game::Game;
use crossterm::{cursor, event, execute, terminal};
use std::io::{stdout, Error, Write};

pub fn setup_terminal() -> Result<(), Error> {
    terminal::enable_raw_mode()?;
    execute!(
        stdout(),
        terminal::EnterAlternateScreen,
        event::EnableMouseCapture,
        cursor::Hide
    )?;
    Ok(())
}

pub fn draw_game(game: &Game) -> Result<(), Error> {
    let mut output = String::new();
    for (y, row) in game.map.tiles.iter().enumerate() {
        for (x, &tile) in row.iter().enumerate() {
            if x == game.player_x && y == game.player_y {
                output.push('@');
            } else {
                output.push(tile);
            }
        }
        output.push_str("\r\n");
    }

    execute!(stdout(), cursor::MoveTo(0, 0))?;
    stdout().write_all(output.as_bytes())?;
    stdout().flush()?;
    Ok(())
}

pub fn cleanup_terminal() -> Result<(), Error> {
    terminal::disable_raw_mode()?;
    execute!(
        stdout(),
        terminal::LeaveAlternateScreen,
        event::DisableMouseCapture,
        cursor::Show
    )?;
    Ok(())
}
