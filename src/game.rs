use crate::map;
use crate::player;

pub struct Game {
    maps: Vec<map::Map>,
    cur_map: usize,
    player: player::Player,
}

impl Game {
    pub fn new() -> Self
    {
        Self {
            maps: Vec::new(),
            cur_map: 0,
            player: player::Player::new(),
        }
    }

    pub fn init_maps(&mut self, map_paths: Vec<&str>)
    {
        for m_path in &map_paths
        {
            self.maps.push(
                map::Map::from_file(m_path).unwrap()
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
