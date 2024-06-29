use super::{common::*, Map};
use crate::{Rect, components::Position};
use rand::Rng;

pub struct SimpleMap {
    pub map: Map,
    pub starting_position: Position,
    pub rooms: Vec<Rect>,
}

impl SimpleMap {
    pub fn new() -> Self {
        Self {
            map: Map::new(80, 24, "Alpha Map"),
            starting_position: Position { x: 0, y: 0 },
            rooms: Vec::new(),
        }
    }

    pub fn simple_map() -> Map {
        const MAX_ROOMS: i32 = 24;
        const MIN_HEIGHT: i32 = 3;
        const MAX_HEIGHT: i32 = 5;
        const MIN_WIDTH: i32 = 6;
        const MAX_WIDTH: i32 = 12;

        let mut map = Map::new(80, 24, "Alpha Map");

        let mut rooms: Vec<Rect> = Vec::new();

        for _ in 0..MAX_ROOMS {
            let w = rand::thread_rng().gen_range(MIN_WIDTH..=MAX_WIDTH);
            let h = rand::thread_rng().gen_range(MIN_HEIGHT..=MAX_HEIGHT);
            let x = rand::thread_rng().gen_range(0..(map.width - w - 1));
            let y = rand::thread_rng().gen_range(0..(map.height - h - 1));

            let new_room = Rect::new(x, y, w, h);
            let mut valid = true;
            for other_room in rooms.iter() {
                if new_room.intersect(other_room) {
                    valid = false;
                }
            }
            if valid {
                make_room(&new_room, &mut map);

                if !rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
                    if rand::thread_rng().gen_range(0..=1) == 0 {
                        make_horizontal_corridor(&mut map, prev_x, new_x, prev_y);
                        make_vertical_corridor(&mut map, prev_y, new_y, new_x);
                    } else {
                        make_vertical_corridor(&mut map, prev_y, new_y, prev_x);
                        make_horizontal_corridor(&mut map, prev_x, new_y, new_x);
                    }
                }

                rooms.push(new_room);
            }
        }

        map
    }
}
