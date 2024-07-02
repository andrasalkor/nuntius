use crate::map::Map;

pub struct World {
    pub map: Map,
}

impl World {
    pub fn new(starting_map: Map) -> Self {
        Self {
            map: starting_map,
        }
    }    
}
