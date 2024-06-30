use crate::ecs::World;
use crossterm::event::{self, KeyCode};
use std::io;
use std::time::Duration;

pub fn handle_input(world: &mut World) -> Result<bool, io::Error> {
    if event::poll(Duration::from_millis(100))? {
        if let event::Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Left => world.move_player(-1, 0),
                KeyCode::Right => world.move_player(1, 0),
                KeyCode::Up => world.move_player(0, -1),
                KeyCode::Down => world.move_player(0, 1),
                KeyCode::Char('q') => return Ok(false),
                _ => {}
            }
        }
    }
    Ok(true)
}
