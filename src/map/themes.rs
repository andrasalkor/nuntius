use super::{Map, TileType};

pub fn tile_glyph(idx: usize, map: &Map) -> char {
    let glyph;

    match map.tiles[idx] {
        TileType::Wall => glyph = '#',
        TileType::Floor => glyph = '.',
        TileType::Corridor => glyph = ' ',
        _ => glyph = ' ',
    }

    glyph
}
