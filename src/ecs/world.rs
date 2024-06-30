use crate::ecs::{
    components::{Player, Position},
    ComponentVec,
};
use crate::map::{Map, tiletype::tile_walkable};

use std::cell::{RefCell, RefMut};

pub struct World {
    pub map: Map,
    entities_count: usize,
    component_vecs: Vec<Box<dyn ComponentVec>>,
}

impl World {
    pub fn new(starting_map: Map) -> Self {
        Self {
            map: starting_map,
            entities_count: 0,
            component_vecs: Vec::new(),
        }
    }
    
    // TODO - This is a system... maybe systems should go elsewhere or have different naming
    pub fn move_player(&mut self, dx: i32, dy: i32) {
        let mut positions = self.borrow_component_vec::<Position>().unwrap();
        let mut players = self.borrow_component_vec::<Player>().unwrap();
        let zip = positions.iter_mut().zip(players.iter_mut());
        let iter =
            zip.filter_map(|(position, player)| Some((position.as_mut()?, player.as_mut()?)));

        for (position, _player) in iter {
            if tile_walkable(self.map.tiles[self.map.xy_idx(position.x + dx, position.y + dy)]) {
                position.x += dx;
                position.y += dy;
            }
        }
    }

    pub fn add_entity(&mut self) -> usize {
        let entity_id = self.entities_count;
        for component_vec in self.component_vecs.iter_mut() {
            component_vec.push_none();
        }
        self.entities_count += 1;
        entity_id
    }

    pub fn add_component_to_entity<ComponentType: 'static>(
        &mut self,
        entity: usize,
        component: ComponentType,
    ) {
        for component_vec in self.component_vecs.iter_mut() {
            if let Some(component_vec) = component_vec
                .as_any_mut()
                .downcast_mut::<RefCell<Vec<Option<ComponentType>>>>()
            {
                component_vec.get_mut()[entity] = Some(component);
                return;
            }
        }

        let mut new_component_vec: Vec<Option<ComponentType>> =
            Vec::with_capacity(self.entities_count);

        for _ in 0..self.entities_count {
            new_component_vec.push(None);
        }

        new_component_vec[entity] = Some(component);
        self.component_vecs
            .push(Box::new(RefCell::new(new_component_vec)));
    }

    pub fn borrow_component_vec<ComponentType: 'static>(
        &self,
    ) -> Option<RefMut<Vec<Option<ComponentType>>>> {
        for component_vec in self.component_vecs.iter() {
            if let Some(component_vec) = component_vec
                .as_any()
                .downcast_ref::<RefCell<Vec<Option<ComponentType>>>>()
            {
                return Some(component_vec.borrow_mut());
            }
        }
        None
    }
}
