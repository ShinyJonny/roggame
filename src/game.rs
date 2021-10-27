use crate::map::Map;

pub struct Game {
    maps: Vec<Map>,
}

impl Game {
    pub fn new() -> Self
    {
        Self {
            maps: Vec::new(),
        }
    }

    pub fn load_maps(&mut self, map_paths: Vec<&str>)
    {
        for m_path in &map_paths
        {
            self.maps.push(
                Map::from_file(m_path).unwrap()
            )
        }

        for m in &self.maps
        {
            m.dump_grid();
        }
    }

    pub fn init_player(&mut self) // TODO
    {
    }

    pub fn start(&mut self) // TODO
    {
    }
}
