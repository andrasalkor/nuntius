// Nuntius
// -------
// This file is responsible for setting up the game
// and making sure it registers all systems and
// everything else needed to be handled.

mod components;
mod ecs;
mod map;
mod systems;

use crate::map::{builder::SimpleMap, MapBuilder}; // This shouldn't be here - refer to TODO below
use components::{Player, Position, Renderable};
use ecs::{World, ECS};
use std::io::Error;
use std::result::Result;
use systems::{
    input_system::handle_input,
    render_system::{render_map, render_message},
    terminal_system::{cleanup_terminal, setup_terminal},
};

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

    // TODO - Rewrite program so we register all systems in the main loop.
    loop {
        render_message(&world)?;
        render_map(&ecs, &world)?;
        if !handle_input(&mut ecs, &world)? {
            break;
        }
    }

    cleanup_terminal()?;
    Ok(())
}
