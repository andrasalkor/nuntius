// Render System
// -------------
// This system is responsible for rendering
// various elements of the game in the terminal.
//
// Currently, it renders the map title
// TODO - change render_title to render_message,
// this is where game messages will have to be
// rendered in the future.

use crate::components::{Position, Renderable};
use crate::ecs::{World, ECS};
use crate::map::themes::tile_glyph;

use crossterm::{cursor, execute};
use std::io::{stdout, Error, Write};

// Temporary feature: render the name of the map
pub fn render_title(world: &World) -> Result<(), Error> {
    let mut output = String::new();
    output.push_str(&world.map.name);
    execute!(stdout(), cursor::MoveTo(0, 0))?;
    stdout().write_all(output.as_bytes())?;
    stdout().flush()?;

    Ok(())
}

pub fn render_map(ecs: &ECS, world: &World) -> Result<(), Error> {
    let mut renderables = ecs.borrow_component_vec::<Renderable>().unwrap();
    let mut positions = ecs.borrow_component_vec::<Position>().unwrap();

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
