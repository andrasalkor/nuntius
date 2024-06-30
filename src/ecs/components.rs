#[derive(PartialEq, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(PartialEq, Clone)]
pub struct Renderable {
    pub glyph: char,
}

pub struct Player;

pub struct Name {
    pub name: String,
}
