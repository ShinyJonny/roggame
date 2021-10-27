pub trait Entity {
    fn name(&self) -> &str;
    fn pos(&self) -> (u32, u32);
    fn place(&mut self, y: u32, x: u32);
}

pub struct Item {
}

pub struct Mob {
    name: String,
    hp: i32,
    pw: i32,
    x_pos: u32,
    y_pos: u32,
}

impl Mob {
    pub fn new(name: &str, hp: i32, pw: i32) -> Self
    {
        Self {
            name: String::from(name),
            hp,
            pw,
            x_pos: 0,
            y_pos: 0,
        }
    }
}

impl Entity for Mob {
    fn name(&self) -> &str
    {
        &self.name
    }

    fn pos(&self) -> (u32, u32)
    {
        (self.y_pos, self.x_pos)
    }

    fn place(&mut self, y: u32, x: u32)
    {
        self.x_pos = x;
        self.y_pos = y;
    }
}
