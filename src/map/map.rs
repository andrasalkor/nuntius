use super::TileType;

#[derive(Clone)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub width: i32,
    pub height: i32,
    pub name: String,
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    pub fn new<S: ToString>(width: i32, height: i32, name: S) -> Self {
        let map_tile_count = (width * height) as usize;

        Self {
            tiles: vec![TileType::Empty; map_tile_count],
            width,
            height,
            name: name.to_string(),
        }
    }
}
