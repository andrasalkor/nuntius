use crate::game::Map;

pub struct Game {
    pub player_x: usize,
    pub player_y: usize,
    pub map: Map,
}

impl Game {
    pub fn new() -> Self {
        Self {
            player_x: 1,
            player_y: 1,
            map: Map::new(),
        }
    }

    pub fn move_player(&mut self, dx: isize, dy: isize) {
        let new_x = (self.player_x as isize + dx) as usize;
        let new_y = (self.player_y as isize + dy) as usize;

        if self.map.can_move(new_x, new_y) {
            self.player_x = new_x;
            self.player_y = new_y;
        }
    }
}
