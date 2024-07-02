use crossterm::{cursor, event, execute, terminal};
use std::io::{stdout, Error};

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
