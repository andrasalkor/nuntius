use super::{common::*, Map, MapBuilder, Rect};
use crate::ecs::Position;
use rand::Rng;

pub struct SimpleMap {
    map: Map,
    starting_position: Position,
    rooms: Vec<Rect>,
}

impl MapBuilder for SimpleMap {
    fn build_map(&mut self) {
        self.simple_map()
    }

    fn get_starting_position(&self) -> Position {
        self.starting_position.clone()
    }

    fn get_map(&self) -> Map {
        self.map.clone()
    }
}

impl SimpleMap {
    pub fn new() -> Self {
        Self {
            map: Map::new(80, 24, "SimpleMap"),
            starting_position: Position { x: 0, y: 0 },
            rooms: Vec::new(),
        }
    }

    fn simple_map(&mut self) {
        const MAX_ROOMS: i32 = 16;
        const MIN_HEIGHT: i32 = 3;
        const MAX_HEIGHT: i32 = 5;
        const MIN_WIDTH: i32 = 6;
        const MAX_WIDTH: i32 = 12;

        for _ in 0..MAX_ROOMS {
            let w = rand::thread_rng().gen_range(MIN_WIDTH..=MAX_WIDTH);
            let h = rand::thread_rng().gen_range(MIN_HEIGHT..=MAX_HEIGHT);
            let x = rand::thread_rng().gen_range(1..(self.map.width - w - 1));
            let y = rand::thread_rng().gen_range(1..(self.map.height - h - 1));

            let new_room = Rect::new(x, y, w, h);
            let mut valid = true;
            for other_room in self.rooms.iter() {
                if new_room.intersect(other_room) {
                    valid = false;
                }
            }
            if valid {
                make_room(&new_room, &mut self.map);

                if !self.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = self.rooms[self.rooms.len() - 1].center();
                    if rand::thread_rng().gen_range(0..=1) == 0 {
                        make_horizontal_corridor(&mut self.map, prev_x, new_x, prev_y);
                        make_vertical_corridor(&mut self.map, prev_y, new_y, new_x);
                    } else {
                        make_vertical_corridor(&mut self.map, prev_y, new_y, prev_x);
                        make_horizontal_corridor(&mut self.map, prev_x, new_y, new_x);
                    }
                }

                self.rooms.push(new_room);
            }
        }
        let start_pos = self.rooms[0].center();
        self.starting_position = Position {
            x: start_pos.0,
            y: start_pos.1,
        };
    }
}
