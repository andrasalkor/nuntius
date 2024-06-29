use super::{Map, TileType};
use crate::Rect;
use std::cmp::{max, min};

pub fn make_room(room: &Rect, map: &mut Map) {
    for y in room.y1..room.y2 {
        for x in room.x1..room.x2 {
            let idx = map.xy_idx(x, y);
            map.tiles[idx] = TileType::Floor;
        }
    }
}

pub fn make_horizontal_corridor(map: &mut Map, x1: i32, x2: i32, y: i32) -> Vec<usize> {
    let mut corridor = Vec::new();
    for x in min(x1, x2)..max(x1, x2) {
        let idx = map.xy_idx(x, y);
        if idx > 0
            && idx < map.width as usize * map.height as usize
            && map.tiles[idx as usize] != TileType::Floor
        {
            map.tiles[idx as usize] = TileType::Corridor;
            corridor.push(idx as usize);
        }
    }
    corridor
}

pub fn make_vertical_corridor(map: &mut Map, y1: i32, y2: i32, x: i32) -> Vec<usize> {
    let mut corridor = Vec::new();
    for y in min(y1, y2)..max(y1, y2) {
        let idx = map.xy_idx(x, y);
        if idx > 0
            && idx < map.width as usize * map.height as usize
            && map.tiles[idx as usize] != TileType::Floor
        {
            map.tiles[idx as usize] = TileType::Corridor;
            corridor.push(idx as usize);
        }
    }
    corridor
}
