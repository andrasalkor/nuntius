pub mod tiletype;
pub use tiletype::TileType;
pub mod builder;
pub mod common;
pub mod rect;
pub mod themes;
pub use rect::Rect;
pub mod map;
pub use map::Map;

use crate::components::Position;

pub trait MapBuilder {
    fn build_map(&mut self);
    fn get_starting_position(&self) -> Position;
    fn get_map(&self) -> Map;
}
