use crate::map::Map;

// Since we have ECS, that is technically a "world", and we can have multiple ECSs
// we don't need a separate world. TODO - Figure out how to inject maps into an ECS
pub struct World {
    pub map: Map,
}

impl World {
    pub fn new(starting_map: Map) -> Self {
        Self { map: starting_map }
    }
}
