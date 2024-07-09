// Render System
// -------------
// This system is responsible for rendering
// various elements of the game in the terminal.
//
// Currently, it renders the map's name where the
// game messages would be rendered, and it renders
// the map view.

use crate::components::{Player, Position, Renderable};
use crate::ecs::{World, ECS};
use crate::map::themes::tile_glyph;

use crossterm::{cursor, execute};
use std::io::{stdout, Error, Write};

const MESSAGE_OFFSET: Position = Position { x: 0, y: 0 };
const MAP_OFFSET: Position = Position { x: 0, y: 1 };

// Temporary feature: renders the name of the map
pub fn render_message(world: &World) -> Result<(), Error> {
    let mut output = String::new();
    output.push_str(&world.map.name);
    execute!(
        stdout(),
        cursor::MoveTo(MESSAGE_OFFSET.x as u16, MESSAGE_OFFSET.y as u16)
    )?;
    stdout().write_all(output.as_bytes())?;
    stdout().flush()?;

    Ok(())
}

pub fn render_map(ecs: &ECS, world: &World) -> Result<(), Error> {
    // Render map based on whether an entity stands on a tile
    let mut output = String::new();

    // Check if there are any renderables with positions
    // TODO - Is there a smarter way of going about this?
    if ecs.borrow_component_vec::<Renderable>().is_some()
        && ecs.borrow_component_vec::<Position>().is_some()
    {
        let mut renderables = ecs.borrow_component_vec::<Renderable>().unwrap();
        let mut positions = ecs.borrow_component_vec::<Position>().unwrap();

        for y in 0..world.map.height {
            for x in 0..world.map.width {
                let iter = renderables.iter_mut().zip(positions.iter_mut()).filter_map(
                    |(renderable, position)| Some((renderable.as_mut()?, position.as_mut()?)),
                );
                for (renderable, position) in iter {
                    if position == (&Position { x: x, y: y }) {
                        output.push(renderable.glyph);
                    } else {
                        output.push(tile_glyph(world.map.xy_idx(x, y), &world.map));
                    }
                }
            }
            output.push_str("\r\n");
        }
    } else {
        // Else just render the map itself
        for y in 0..world.map.height {
            for x in 0..world.map.width {
                output.push(tile_glyph(world.map.xy_idx(x, y), &world.map));
            }
            output.push_str("\r\n");
        }
    }
    // Write output to screen
    execute!(
        stdout(),
        cursor::MoveTo(MAP_OFFSET.x as u16, MAP_OFFSET.y as u16)
    )?;
    stdout().write_all(output.as_bytes())?;
    stdout().flush()?;

    // Get cursor to the last player's position
    // Check if player exists with a position first
    if ecs.borrow_component_vec::<Player>().is_some() {
        execute!(stdout(), cursor::Show)?;
        let mut players = ecs.borrow_component_vec::<Player>().unwrap();
        let mut positions = ecs.borrow_component_vec::<Position>().unwrap();
        let iter = players
            .iter_mut()
            .zip(positions.iter_mut())
            .filter_map(|(player, position)| Some((player.as_mut()?, position.as_mut()?)));
        for (_player, position) in iter {
            execute!(
                stdout(),
                cursor::MoveTo(
                    position.x as u16 + MAP_OFFSET.x as u16,
                    position.y as u16 + MAP_OFFSET.y as u16
                )
            )?;
        }
    } else {
        // Else hide cursor since there is no player to follow
        execute!(stdout(), cursor::Hide)?;
    }
    Ok(())
}
