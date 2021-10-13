use crate::entity;

pub struct Map {
    grid: Vec<Vec<u8>>,
    objects: Vec<Box<dyn entity::Entity>>,
}

impl Map {
    pub fn new() -> Self
    {
        Self {
            grid: Vec::new(),
            objects: Vec::new(),
        }
    }

    pub fn from_file(m_path: &str) -> Result<Self, std::io::Error>
    {
        let mut m = Self::new();
        m.load(m_path)?;

        Ok(m)
    }

    pub fn load(&mut self, file: &str) -> Result <(), std::io::Error>
    {
        // TODO
    }

    pub fn size(&self) -> (u32, u32)
    {
        if self.grid.len() == 0 {
            (0, 0)
        } else {
            (self.grid.len() as u32, self.grid[0].len() as u32)
        }
    }
}
