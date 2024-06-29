mod input;
mod map;
mod rect;
pub use rect::Rect;
mod rendering;
mod world;
mod components;

use input::handle_input;
use rendering::{cleanup_terminal, render_map, setup_terminal};
use std::io::Error;
use std::result::Result;
use world::World;

fn main() -> Result<(), Error> {
    setup_terminal()?;

    let mut world = World::new();

    loop {
        render_map(&world)?;
        if !handle_input(&mut world)? {
            break;
        }
    }

    cleanup_terminal()?;
    Ok(())
}
