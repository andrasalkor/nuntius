mod ecs;
mod components;
mod systems;
mod map;

use crate::map::{builder::SimpleMap, MapBuilder}; // This shouldn't be here - refer to TODO below
use ecs::{World, ECS};
use components::{Player, Position, Renderable};
use systems::{input_system::handle_input, terminal_system::{setup_terminal, cleanup_terminal}, render_system::{render_title, render_map}};
use std::io::Error;
use std::result::Result;

fn main() -> Result<(), Error> {
    setup_terminal()?;

    // Temporary solution for map generation
    // TODO - Refactor code so this is handled by its proper system
    let mut starting_map = SimpleMap::new();
    starting_map.build_map();
    let world = World::new(starting_map.get_map());

    // Initialise ECS and add player entity
    let mut ecs = ECS::new();
    let player_entity = ecs.add_entity();
    ecs.add_component_to_entity(player_entity, Renderable { glyph: '@' });
    ecs.add_component_to_entity(player_entity, Player);
    ecs.add_component_to_entity(
        player_entity,
        Position {
            x: starting_map.get_starting_position().x,
            y: starting_map.get_starting_position().y,
        },
    );

    loop {
        render_title(&world)?;
        render_map(&ecs, &world)?;
        if !handle_input(&mut ecs, &world)? {
            break;
        }
    }

    cleanup_terminal()?;
    Ok(())
}
