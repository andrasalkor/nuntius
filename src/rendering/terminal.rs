use crate::ecs::{
    components::{Position, Renderable},
    World,
};
use crate::map::themes::tile_glyph;
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
    let mut renderables = world.borrow_component_vec::<Renderable>().unwrap();
    let mut positions = world.borrow_component_vec::<Position>().unwrap();
    let zip = renderables.iter_mut().zip(positions.iter_mut());

    let mut output = String::new();
    for y in 0..world.map.height {
        for x in 0..world.map.width {
            // TODO - Move this outside
            let zip = renderables.iter_mut().zip(positions.iter_mut());
            let iter = zip.filter_map(|(renderable, position)| {
                Some((renderable.as_mut()?, position.as_mut()?))
            });
            for (renderable, position) in iter {
                if position == (&Position { x: x, y: y }) {
                    output.push(renderable.glyph);
                } else {
                    // TODO - Leave this inside with improved logic
                    output.push(tile_glyph(world.map.xy_idx(x, y), &world.map));
                }
            }
        }
        output.push_str("\r\n");
    }
    execute!(stdout(), cursor::MoveTo(0, 1))?;
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
