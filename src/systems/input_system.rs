// Input System
// ------------
// This system is responsible for handling
// user input.

use crate::ecs::{World, ECS};
use crossterm::event::{self, KeyCode};
use std::io;
use std::time::Duration;

// TODO - Make it not rely on a World.
pub fn handle_input(ecs: &mut ECS, world: &World) -> Result<bool, io::Error> {
    if event::poll(Duration::from_millis(100))? {
        if let event::Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Left => ecs.move_player(world, -1, 0),
                KeyCode::Right => ecs.move_player(world, 1, 0),
                KeyCode::Up => ecs.move_player(world, 0, -1),
                KeyCode::Down => ecs.move_player(world, 0, 1),
                KeyCode::Char('q') => return Ok(false),
                _ => {}
            }
        }
    }
    Ok(true)
}
