use crate::entity;

pub struct Player {
    pub name: String,
    x_pos: u32,
    y_pos: u32,
    invent: Vec<entity::Item>,
}

impl Player {
    pub fn new() -> Self
    {
        Self {
            name: String::new(),
            x_pos: 0,
            y_pos: 0,
            invent: Vec::new(),
        }
    }
}

impl entity::Entity for Player {
    fn pos(&self) -> (u32, u32)
    {
        (self.y_pos, self.x_pos)
    }

    fn place(&mut self, y: u32, x: u32)
    {
        self.x_pos = x;
        self.y_pos = y;
    }

    fn name(&self) -> &str {
        &self.name
    }
}
