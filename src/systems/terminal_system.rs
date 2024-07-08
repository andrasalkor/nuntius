// Terminal System
// ---------------
// This system is responsible for setting up and
// clearing the terminal the game is running in.

use crossterm::{event, execute, terminal};
use std::io::{stdout, Error};

pub fn setup_terminal() -> Result<(), Error> {
    terminal::enable_raw_mode()?;
    execute!(
        stdout(),
        terminal::EnterAlternateScreen,
        event::EnableMouseCapture
    )?;
    Ok(())
}

pub fn cleanup_terminal() -> Result<(), Error> {
    terminal::disable_raw_mode()?;
    execute!(
        stdout(),
        terminal::LeaveAlternateScreen,
        event::DisableMouseCapture
    )?;
    Ok(())
}
