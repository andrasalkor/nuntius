use super::{Map, Rect, TileType};
use std::cmp::{max, min};

pub fn make_room(room: &Rect, map: &mut Map) {
    for y in room.y1..room.y2 {
        for x in room.x1..room.x2 {
            let idx = map.xy_idx(x, y);
            map.tiles[idx] = TileType::Floor;
        }
    }

    for y in room.y1..=room.y2 {
        if room.x1 > 0 {
            let idx = map.xy_idx(room.x1 - 1, y);
            if map.tiles[idx] == TileType::Empty {
                map.tiles[idx] = TileType::Wall;
            }
        }
        if room.x2 < map.width {
            let idx = map.xy_idx(room.x2, y);
            if map.tiles[idx] == TileType::Empty {
                map.tiles[idx] = TileType::Wall;
            }
        }

        for x in room.x1 - 1..=room.x2 {
            if room.y1 > 0 {
                let idx = map.xy_idx(x, room.y1 - 1);
                if map.tiles[idx] == TileType::Empty {
                    map.tiles[idx] = TileType::Wall;
                }
            }
            if room.y2 < map.height {
                let idx = map.xy_idx(x, room.y2);
                if map.tiles[idx] == TileType::Empty {
                    map.tiles[idx] = TileType::Wall;
                }
            }
        }
    }
}

pub fn make_horizontal_corridor(map: &mut Map, x1: i32, x2: i32, y: i32) -> Vec<usize> {
    let mut corridor = Vec::new();
    for x in min(x1, x2)..=max(x1, x2) {
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
    for y in min(y1, y2)..=max(y1, y2) {
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
