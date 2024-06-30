#[derive(PartialEq, Clone, Copy)]
pub enum TileType {
    Wall,
    Floor,
    Corridor,
    Empty,
}

pub fn tile_walkable(tt: TileType) -> bool {
    match tt {
        TileType::Floor | TileType::Corridor => true,
        _ => false,
    }
}

// TODO - Implement FOV and seen/unseen discovered/undiscovered tiles
// pub fn tile_dark(tt: TileType) -> bool {
//     match tt {
//         TileType::Corridor => true,
//         _ => false,
//     }
// }
