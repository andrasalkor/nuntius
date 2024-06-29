use crate::map::themes::tile_glyph;
use crate::world::World;
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

pub fn render_map(world: &World) -> Result<(), Error> {
    let mut output = String::new();
    output.push_str(&world.map.name);
    output.push_str("\r\n");
    for y in 0..world.map.height {
        for x in 0..world.map.width {
            output.push(tile_glyph(world.map.xy_idx(x, y), &world.map));
        }
        output.push_str("\r\n");
    }
    output.push_str(&world.map.name);
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
