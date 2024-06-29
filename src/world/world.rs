use crate::map::builder::SimpleMap;
use crate::map::Map;

pub struct World {
    pub map: Map,
}

impl World {
    pub fn new() -> Self {
        Self {
            map: SimpleMap::simple_map(),
        }
    }

    pub fn move_player(&mut self, _dx: isize, _dy: isize) {
        todo!("Working on map render... controls after.")
    }
}
