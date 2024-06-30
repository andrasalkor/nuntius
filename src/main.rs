mod ecs;
mod input;
mod map;
mod rendering;

use crate::map::{builder::SimpleMap, MapBuilder}; // This shouldn't be here - refer to TODO below
use ecs::{
    components::{Player, Position, Renderable},
    World,
};
use input::handle_input;
use rendering::{cleanup_terminal, render_title, render_map, setup_terminal};
use std::io::Error;
use std::result::Result;

fn main() -> Result<(), Error> {
    setup_terminal()?;

    // Temporary solution for map generation
    // TODO - Refactor code so this is handled by its proper system
    let mut starting_map = SimpleMap::new();
    starting_map.build_map();
    let mut world = World::new(starting_map.get_map());

    let player_entity = world.add_entity();
    world.add_component_to_entity(player_entity, Renderable { glyph: '@' });
    world.add_component_to_entity(player_entity, Player);
    world.add_component_to_entity(
        player_entity,
        Position {
            x: starting_map.get_starting_position().x,
            y: starting_map.get_starting_position().y,
        },
    );

    loop {
        render_title(&world)?;
        render_map(&world)?;
        if !handle_input(&mut world)? {
            break;
        }
    }

    cleanup_terminal()?;
    Ok(())
}
