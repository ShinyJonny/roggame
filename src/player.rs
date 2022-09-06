#[derive(Clone, Copy)]
pub struct Pos {
    pub y: u32,
    pub x: u32,
}

pub struct Player {
    pub name: String,
    pub pos: Pos,
}

impl Player {
    pub fn new() -> Self
    {
        Self {
            name: String::new(),
            pos: Pos {y: 0, x: 0},
        }
    }
}
