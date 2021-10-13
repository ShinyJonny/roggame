pub trait Entity {
    // TODO:
    // name
    // id
    // symbol
    // type // Returns enum that has type-specific methods & attributes.
    fn pos(&self) -> (u32, u32);
    fn place(&mut self, y: u32, x: u32);
}

pub trait Item : Entity {
    fn weight(&self) -> i32;
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

    fn attack(&self, enemy: Mob)
    {
        // TODO
    }
}

impl Entity for Mob {
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
